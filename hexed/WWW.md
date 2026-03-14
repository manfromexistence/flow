Looking at the code, I can identify several core issues:

1. **Circular dependency**: Screen dimensions stored in parent state but derived from container size, causing cascading re-renders.
2. **Layout math mismatch**: Rendered screen widths differ from what the x-offset calculation assumes (some use `containerWidth`, others `containerWidth - 32`).
3. **Fragile identity**: `activeScreenType` breaks when multiple screens share a type.
4. **Drag decision**: Mixes pixels and px/s without normalization.
5. **Resize coupling**: Manually adjusting `x` during resize is complex and brittle when the slot-based approach eliminates the need entirely.

Here's the full refactored solution:

---

### New file: `components/screens/carousel-math.ts`

```ts
// ─────────────────────────────────────────────────────────────
// Pure, stateless functions for carousel layout & snap math.
// Every function is independently testable with no React dependency.
// ─────────────────────────────────────────────────────────────

export interface CarouselConfig {
  /** Gap in px between adjacent screen slots. */
  gap: number;
  /** Inset padding (per axis) between container edge and default screen edge. */
  padding: number;
  /** Absolute velocity (px/s) that alone triggers a screen change. */
  velocityThreshold: number;
  /** Fraction of container width that alone triggers a screen change. */
  offsetRatioThreshold: number;
  /**
   * When neither threshold is met individually, a combined check fires if
   * velocity > velocityThreshold × combinedVelocityFactor AND
   * offset  > containerWidth × offsetRatioThreshold × combinedOffsetFactor.
   */
  combinedVelocityFactor: number;
  combinedOffsetFactor: number;
  /** Elastic overscroll factor at carousel edges (0 = none, 1 = free). */
  dragElastic: number;
  /** Minimum screen dimensions (resize clamp). */
  minScreenWidth: number;
  minScreenHeight: number;
}

export const DEFAULT_CAROUSEL_CONFIG: CarouselConfig = {
  gap: 8,
  padding: 32,
  velocityThreshold: 500,
  offsetRatioThreshold: 0.25,
  combinedVelocityFactor: 0.45,
  combinedOffsetFactor: 0.4,
  dragElastic: 0.08,
  minScreenWidth: 400,
  minScreenHeight: 300,
};

// ── Layout helpers ───────────────────────────────────────────

export interface Size {
  width: number;
  height: number;
}

/**
 * The "logical" size a screen occupies when it has not been manually resized.
 * Fills the container minus a small padding so content doesn't press against
 * the edges.
 */
export function logicalScreenSize(
  containerWidth: number,
  containerHeight: number,
  config: Pick<CarouselConfig, "padding" | "minScreenWidth" | "minScreenHeight">,
): Size {
  return {
    width: Math.max(containerWidth - config.padding, config.minScreenWidth),
    height: Math.max(containerHeight - config.padding, config.minScreenHeight),
  };
}

/**
 * Clamp a manually-resized screen so it never overflows the container
 * or drops below the minimum.
 */
export function clampScreenSize(
  width: number,
  height: number,
  containerWidth: number,
  containerHeight: number,
  config: Pick<CarouselConfig, "padding" | "minScreenWidth" | "minScreenHeight">,
): Size {
  const maxW = Math.max(containerWidth - config.padding, config.minScreenWidth);
  const maxH = Math.max(containerHeight - config.padding, config.minScreenHeight);
  return {
    width: Math.max(config.minScreenWidth, Math.min(width, maxW)),
    height: Math.max(config.minScreenHeight, Math.min(height, maxH)),
  };
}

// ── Offset / constraint helpers ──────────────────────────────

/**
 * The x translation that places the slot at `index` flush with
 * the left edge of the container.  Because every slot is exactly
 * `containerWidth` wide, the formula is trivially linear.
 */
export function targetXForIndex(
  index: number,
  containerWidth: number,
  gap: number,
): number {
  return -(index * (containerWidth + gap));
}

/**
 * Hard drag boundaries so the strip can never be pulled more than
 * one slot past either end (the elastic factor softens this further).
 */
export function dragConstraints(
  screenCount: number,
  containerWidth: number,
  gap: number,
): { left: number; right: number } {
  return {
    left: -((screenCount - 1) * (containerWidth + gap)),
    right: 0,
  };
}

// ── Snap decision ────────────────────────────────────────────

/**
 * Given the drag offset (px, negative = dragged left) and velocity
 * (px/s), decide which screen index to snap to.
 *
 * Three independent "triggers" are checked:
 *  1. Pure velocity  — a fast fling regardless of distance.
 *  2. Pure distance  — a slow but long drag.
 *  3. Combined       — moderate speed + moderate distance.
 *
 * If none fire, we rubber-band back to `currentIndex`.
 */
export function resolveSnapIndex(
  currentIndex: number,
  screenCount: number,
  offsetX: number,
  velocityX: number,
  containerWidth: number,
  config: Pick<
    CarouselConfig,
    | "velocityThreshold"
    | "offsetRatioThreshold"
    | "combinedVelocityFactor"
    | "combinedOffsetFactor"
  >,
): number {
  if (screenCount <= 1) return 0;
  if (containerWidth <= 0) return currentIndex;

  // Negative offset ⟹ dragged left ⟹ next screen (+1)
  const direction = offsetX < 0 ? 1 : -1;
  const absOffset = Math.abs(offsetX);
  const absVelocity = Math.abs(velocityX);

  const byVelocity = absVelocity > config.velocityThreshold;
  const byDistance = absOffset > containerWidth * config.offsetRatioThreshold;
  const byCombined =
    absVelocity > config.velocityThreshold * config.combinedVelocityFactor &&
    absOffset > containerWidth * config.offsetRatioThreshold * config.combinedOffsetFactor;

  if (!byVelocity && !byDistance && !byCombined) return currentIndex;

  return Math.max(0, Math.min(screenCount - 1, currentIndex + direction));
}
```

---

### New file: `components/screens/hooks/use-container-size.ts`

```ts
import { useEffect, useState, type RefObject } from "react";

export interface ContainerSize {
  width: number;
  height: number;
}

const EMPTY: ContainerSize = { width: 0, height: 0 };

/**
 * Tracks the bounding-client-rect size of an element via ResizeObserver.
 * Returns a stable object reference when the size hasn't changed.
 */
export function useContainerSize(
  ref: RefObject<HTMLElement | null>,
): ContainerSize {
  const [size, setSize] = useState<ContainerSize>(EMPTY);

  useEffect(() => {
    const el = ref.current;
    if (!el) return;

    const sync = () => {
      const { width, height } = el.getBoundingClientRect();
      setSize((prev) =>
        prev.width === width && prev.height === height
          ? prev
          : { width, height },
      );
    };

    sync();

    const observer = new ResizeObserver(sync);
    observer.observe(el);
    return () => observer.disconnect();
  }, [ref]);

  return size;
}
```

---

### New file: `components/screens/hooks/use-carousel.ts`

```ts
import { useCallback, useEffect, useRef, useState } from "react";
import { animate, useMotionValue, type PanInfo } from "motion/react";

import type { CarouselConfig, Size } from "../carousel-math";
import {
  clampScreenSize,
  DEFAULT_CAROUSEL_CONFIG,
  dragConstraints,
  logicalScreenSize,
  resolveSnapIndex,
  targetXForIndex,
} from "../carousel-math";
import type { ContainerSize } from "./use-container-size";

// ── Types ────────────────────────────────────────────────────

export interface UseCarouselOptions {
  screenCount: number;
  activeIndex: number;
  containerSize: ContainerSize;
  config?: CarouselConfig;
  onActiveIndexChange: (index: number) => void;
}

export interface UseCarouselReturn {
  /** Motion value driving the horizontal translation of the strip. */
  x: ReturnType<typeof useMotionValue<number>>;
  /** Current carousel config (merged with defaults). */
  config: CarouselConfig;
  /** Whether the user is currently dragging. */
  isDragging: boolean;
  /** Whether the active screen is currently being resized. */
  isResizing: boolean;
  /** Constraints for the motion drag. */
  constraints: { left: number; right: number };
  /** The default (non-resized) screen size derived from the container. */
  logicalSize: Size;

  /**
   * Effective size for a given screen index, accounting for any
   * manual resize override on the active screen.
   */
  effectiveScreenSize: (index: number) => Size;

  /** Call on drag start (wire to motion's `onDragStart`). */
  handleDragStart: () => void;
  /** Call on drag end (wire to motion's `onDragEnd`). */
  handleDragEnd: (_event: MouseEvent | TouchEvent | PointerEvent, info: PanInfo) => void;

  /** Call when the user starts resizing the active screen. */
  handleResizeStart: () => void;
  /**
   * Call during resize with the *new* absolute width/height
   * (not the delta).
   */
  handleResize: (width: number, height: number) => void;
  /** Call when resize ends with the final absolute dimensions. */
  handleResizeStop: (width: number, height: number) => void;
}

// ── Spring config for snap-back animation ────────────────────

const SNAP_SPRING = { type: "spring" as const, stiffness: 300, damping: 30 };

// ── Hook ─────────────────────────────────────────────────────

export function useCarousel({
  screenCount,
  activeIndex,
  containerSize,
  config: configOverride,
  onActiveIndexChange,
}: UseCarouselOptions): UseCarouselReturn {
  const config = configOverride ?? DEFAULT_CAROUSEL_CONFIG;
  const x = useMotionValue(0);

  const [isDragging, setIsDragging] = useState(false);
  const [isResizing, setIsResizing] = useState(false);

  // We store only the active screen's resize override.
  // Cleared on container resize or active-index change.
  const [sizeOverride, setSizeOverride] = useState<Size | null>(null);

  const prevContainerWidthRef = useRef(0);

  // ── Derived values ──────────────────────────────────────

  const logicalSize = logicalScreenSize(
    containerSize.width,
    containerSize.height,
    config,
  );

  const constraints = dragConstraints(
    screenCount,
    containerSize.width,
    config.gap,
  );

  // ── Clear override when container width changes ─────────

  useEffect(() => {
    if (containerSize.width <= 0) return;
    const delta = Math.abs(
      containerSize.width - prevContainerWidthRef.current,
    );
    if (prevContainerWidthRef.current > 0 && delta > 2) {
      setSizeOverride(null);
    }
    prevContainerWidthRef.current = containerSize.width;
  }, [containerSize.width]);

  // ── Clear override when active screen changes ───────────

  const prevActiveIndexRef = useRef(activeIndex);
  useEffect(() => {
    if (prevActiveIndexRef.current !== activeIndex) {
      setSizeOverride(null);
      prevActiveIndexRef.current = activeIndex;
    }
  }, [activeIndex]);

  // ── Animate to the active screen whenever relevant deps change ──

  useEffect(() => {
    if (isDragging || isResizing) return;
    if (containerSize.width <= 0) return;

    const target = targetXForIndex(activeIndex, containerSize.width, config.gap);
    animate(x, target, SNAP_SPRING);
  }, [activeIndex, containerSize.width, isDragging, isResizing, config.gap, x]);

  // ── Screen size accessor ────────────────────────────────

  const effectiveScreenSize = useCallback(
    (index: number): Size => {
      if (index === activeIndex && sizeOverride) {
        return clampScreenSize(
          sizeOverride.width,
          sizeOverride.height,
          containerSize.width,
          containerSize.height,
          config,
        );
      }
      return logicalSize;
    },
    [activeIndex, sizeOverride, logicalSize, containerSize, config],
  );

  // ── Drag handlers ───────────────────────────────────────

  const handleDragStart = useCallback(() => setIsDragging(true), []);

  const handleDragEnd = useCallback(
    (_event: MouseEvent | TouchEvent | PointerEvent, info: PanInfo) => {
      setIsDragging(false);

      const newIndex = resolveSnapIndex(
        activeIndex,
        screenCount,
        info.offset.x,
        info.velocity.x,
        containerSize.width,
        config,
      );

      if (newIndex !== activeIndex) {
        onActiveIndexChange(newIndex);
      } else {
        // Snap back to current even if the index didn't change,
        // in case the strip drifted during the drag.
        const target = targetXForIndex(
          activeIndex,
          containerSize.width,
          config.gap,
        );
        animate(x, target, SNAP_SPRING);
      }
    },
    [activeIndex, screenCount, containerSize.width, config, onActiveIndexChange, x],
  );

  // ── Resize handlers ─────────────────────────────────────

  const handleResizeStart = useCallback(() => setIsResizing(true), []);

  const handleResize = useCallback(
    (width: number, height: number) => {
      setSizeOverride({ width, height });
    },
    [],
  );

  const handleResizeStop = useCallback(
    (width: number, height: number) => {
      setIsResizing(false);
      const clamped = clampScreenSize(
        width,
        height,
        containerSize.width,
        containerSize.height,
        config,
      );
      setSizeOverride(clamped);
    },
    [containerSize, config],
  );

  return {
    x,
    config,
    isDragging,
    isResizing,
    constraints,
    logicalSize,
    effectiveScreenSize,
    handleDragStart,
    handleDragEnd,
    handleResizeStart,
    handleResize,
    handleResizeStop,
  };
}
```

---

### New file: `components/screens/hooks/index.ts`

```ts
export { useContainerSize } from "./use-container-size";
export { useCarousel } from "./use-carousel";
export type { UseCarouselReturn, UseCarouselOptions } from "./use-carousel";
export type { ContainerSize } from "./use-container-size";
```

---

### Updated: `components/screens/types.ts`

```ts
export type ScreenType = "browser" | "terminal" | "code" | "welcome";

/**
 * A screen in the carousel.
 *
 * Dimensions are intentionally *not* stored here — they are derived
 * from the carousel container size and managed by `useCarousel`.
 * This avoids circular state dependencies between parent-level
 * screen state and layout-level container measurements.
 */
export interface Screen {
  id: string;
  type: ScreenType;
  title: string;
}

// Re-export carousel config so consumers don't need a second import.
export type { CarouselConfig } from "./carousel-math";
export { DEFAULT_CAROUSEL_CONFIG } from "./carousel-math";
```

---

### Updated: `components/screens/screen-carousel.tsx`

```tsx
"use client";

import { motion } from "motion/react";
import { Resizable } from "re-resizable";
import { useRef } from "react";

import { cn } from "@/lib/utils";

import { BrowserScreen } from "./browser-screen";
import { CodeScreen } from "./code-screen";
import { useCarousel, useContainerSize } from "./hooks";
import { TerminalScreen } from "./terminal-screen";
import type { Screen, ScreenType } from "./types";
import { WelcomeScreen } from "./welcome-screen";

// ── Props ────────────────────────────────────────────────────

interface ScreenCarouselProps {
  /** ID of the currently active screen. */
  activeScreenId: string;
  screens: Screen[];
  /** Called with the *new* active screen ID after a drag or tap. */
  onScreenChange: (id: string) => void;
}

// ── Screen renderer ──────────────────────────────────────────

function renderScreenContent(type: ScreenType) {
  switch (type) {
    case "terminal":
      return <TerminalScreen />;
    case "code":
      return <CodeScreen />;
    case "browser":
      return <BrowserScreen />;
    case "welcome":
    default:
      return <WelcomeScreen />;
  }
}

// ── Component ────────────────────────────────────────────────

export function ScreenCarousel({
  activeScreenId,
  screens,
  onScreenChange,
}: ScreenCarouselProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const containerSize = useContainerSize(containerRef);

  const activeIndex = Math.max(
    0,
    screens.findIndex((s) => s.id === activeScreenId),
  );

  const {
    x,
    config,
    isDragging,
    isResizing,
    constraints,
    logicalSize,
    effectiveScreenSize,
    handleDragStart,
    handleDragEnd,
    handleResizeStart,
    handleResize,
    handleResizeStop,
  } = useCarousel({
    screenCount: screens.length,
    activeIndex,
    containerSize,
    onActiveIndexChange: (newIndex) => onScreenChange(screens[newIndex].id),
  });

  // Track size at resize start so we can compute absolute values
  // from the delta that re-resizable gives us.
  const resizeStartSizeRef = useRef({ width: 0, height: 0 });

  const ready = containerSize.width > 0 && containerSize.height > 0;

  return (
    <div ref={containerRef} className="relative h-full w-full overflow-hidden">
      {ready && (
        <>
          {/* ── Draggable strip ──────────────────────────── */}
          <motion.div
            className="flex h-full"
            style={{ x, gap: config.gap }}
            drag={isResizing ? false : "x"}
            dragConstraints={constraints}
            dragElastic={config.dragElastic}
            onDragStart={handleDragStart}
            onDragEnd={handleDragEnd}
          >
            {screens.map((screen, index) => {
              const isActive = index === activeIndex;
              const size = effectiveScreenSize(index);

              return (
                <div
                  key={screen.id}
                  className="flex shrink-0 items-center justify-center"
                  style={{
                    width: containerSize.width,
                    height: containerSize.height,
                  }}
                >
                  {isActive ? (
                    <Resizable
                      size={{ width: size.width, height: size.height }}
                      onResizeStart={() => {
                        resizeStartSizeRef.current = {
                          width: size.width,
                          height: size.height,
                        };
                        handleResizeStart();
                      }}
                      onResize={(_e, _dir, _ref, delta) => {
                        handleResize(
                          resizeStartSizeRef.current.width + delta.width,
                          resizeStartSizeRef.current.height + delta.height,
                        );
                      }}
                      onResizeStop={(_e, _dir, _ref, delta) => {
                        handleResizeStop(
                          resizeStartSizeRef.current.width + delta.width,
                          resizeStartSizeRef.current.height + delta.height,
                        );
                      }}
                      minWidth={config.minScreenWidth}
                      minHeight={config.minScreenHeight}
                      maxWidth={logicalSize.width}
                      maxHeight={logicalSize.height}
                      className="relative"
                      enable={{
                        top: true,
                        right: true,
                        bottom: true,
                        left: true,
                        topRight: true,
                        bottomRight: true,
                        bottomLeft: true,
                        topLeft: true,
                      }}
                    >
                      <motion.div
                        initial={{ scale: 0.95, opacity: 0 }}
                        animate={{ scale: 1, opacity: 1 }}
                        className="h-full w-full"
                      >
                        {renderScreenContent(screen.type)}
                      </motion.div>
                    </Resizable>
                  ) : (
                    <div
                      style={{
                        width: logicalSize.width,
                        height: logicalSize.height,
                      }}
                    >
                      {renderScreenContent(screen.type)}
                    </div>
                  )}
                </div>
              );
            })}
          </motion.div>

          {/* ── Dot indicators ───────────────────────────── */}
          <div className="absolute bottom-4 left-1/2 flex -translate-x-1/2 gap-2">
            {screens.map((screen, index) => (
              <button
                key={screen.id}
                onClick={() => onScreenChange(screen.id)}
                className={cn(
                  "h-2 rounded-full transition-all",
                  index === activeIndex
                    ? "w-8 bg-primary"
                    : "w-2 bg-muted-foreground/30 hover:bg-muted-foreground/50",
                )}
              />
            ))}
          </div>
        </>
      )}
    </div>
  );
}
```

---

### Updated: `components/browser/browser-content.tsx`

```tsx
import { useState } from "react";
import { MacOSDock } from "@/components/screens/macos-dock";
import { ScreenCarousel } from "@/components/screens/screen-carousel";
import type { Screen, ScreenType } from "@/components/screens/types";

interface BrowserContentProps {
  sidebarExpanded: boolean;
}

const INITIAL_SCREENS: Screen[] = [
  { id: "welcome", type: "welcome", title: "Welcome" },
  { id: "terminal", type: "terminal", title: "Terminal" },
  { id: "code", type: "code", title: "Code Editor" },
  { id: "browser", type: "browser", title: "Browser" },
];

export function BrowserContent({ sidebarExpanded }: BrowserContentProps) {
  const [screens, setScreens] = useState<Screen[]>(INITIAL_SCREENS);
  const [activeScreenId, setActiveScreenId] = useState<string>("welcome");

  // The dock still operates on ScreenType for icon highlighting.
  // Resolve the type from the active ID for the dock, and
  // convert a dock-initiated type change back to an ID.
  const activeScreenType: ScreenType =
    screens.find((s) => s.id === activeScreenId)?.type ?? "welcome";

  const handleDockScreenChange = (type: ScreenType) => {
    const target = screens.find((s) => s.type === type);
    if (target) setActiveScreenId(target.id);
  };

  const handleAddScreen = () => {
    const newScreen: Screen = {
      id: `screen-${Date.now()}`,
      type: "welcome",
      title: `New Screen ${screens.length + 1}`,
    };
    setScreens((prev) => [...prev, newScreen]);
    setActiveScreenId(newScreen.id);
  };

  const handleToggleViewMode = () => {
    // TODO: Implement grid view mode
    console.log("Toggle view mode");
  };

  return (
    <div className="relative flex h-full w-full items-center justify-center overflow-hidden">
      <MacOSDock
        activeScreen={activeScreenType}
        onScreenChange={handleDockScreenChange}
        onAddScreen={handleAddScreen}
        onToggleViewMode={handleToggleViewMode}
        sidebarExpanded={sidebarExpanded}
      />
      <ScreenCarousel
        activeScreenId={activeScreenId}
        screens={screens}
        onScreenChange={setActiveScreenId}
      />
    </div>
  );
}
```

---

### Updated: `components/screens/index.ts`

```ts
export * from "./browser-screen";
export * from "./code-screen";
export * from "./macos-dock";
export * from "./screen-carousel";
export * from "./terminal-screen";
export * from "./types";
export * from "./carousel-math";
export * from "./welcome-screen";
export * from "./hooks";
```

---

### `components/screens/macos-dock.tsx` — unchanged

No structural changes needed. It already operates on `ScreenType` for highlighting and receives callbacks. It remains decoupled from the carousel internals.

### Screen components (`welcome-screen.tsx`, `terminal-screen.tsx`, `browser-screen.tsx`, `code-screen.tsx`) — unchanged

No changes needed. They are purely presentational and fill their parent via `h-full w-full`.

---

## Summary of architectural changes

### What was wrong & what changed

| Problem | Before | After |
|---|---|---|
| **Circular state** — `screens[].width/height` stored in parent, but derived from container | `useEffect` writes parent state on every container resize; `screens` is in its own dependency array → potential infinite loops, double renders | Dimensions removed from `Screen`. The carousel derives sizes internally via `useCarousel` → zero upward state flow for sizing |
| **Layout slot mismatch** — rendered width ≠ offset-calculation width | Active screens used `screen.width`, non-active used `containerWidth - 32`, but x-offset assumed `containerWidth` | Every slot is exactly `containerWidth`. Screens are centered *within* their slot by CSS flexbox. `targetXForIndex` is a trivial `-(i × (W + gap))` — no per-screen width corrections needed |
| **x adjustment during resize** | Manual `x.set()` inside the `onResize` callback tried to keep centering correct | Unnecessary — the slot width is constant, and the `Resizable` is centered in the slot by its flex parent. No x adjustment needed at all |
| **Fragile identity** | `activeScreenType` breaks with duplicate types | Changed to `activeScreenId`. The dock bridge converts type↔id at the `BrowserContent` level |
| **Drag decision** | `swipePower = offset × 0.5 + velocity × 0.8` mixed units | Three independent, clearly named triggers: `byVelocity`, `byDistance`, `byCombined` — each with documented thresholds |
| **Monolithic component** | All logic in one 200-line component | `carousel-math.ts` (pure functions, testable), `use-container-size` (reusable), `use-carousel` (all stateful carousel logic), `ScreenCarousel` (thin render shell) |
| **Stale sizes after sidebar toggle** | Width delta > 10 px check + re-writing all screen objects | ResizeObserver drives `containerSize`; `logicalScreenSize` is derived on every render; override is cleared when container width changes |
| **Resize throttle hack** | `setTimeout(…, 16)` with manual cleanup | Removed. React 18+ batches `setState` calls automatically. `re-resizable`'s `onResize` fires at frame rate; we just set state directly |

### Key design decisions

1. **Fixed-width slots**: Each screen occupies a slot that is always `containerWidth` pixels wide, regardless of screen content size. This makes the x-offset formula trivially linear and eliminates an entire class of centering/drift bugs.

2. **Size override lives in the hook, not the parent**: Only the active screen can be resized, and the override is a single `{ width, height } | null` that auto-clears on container resize or active-index change.

3. **Pure math in a separate module**: `carousel-math.ts` has zero React imports. You can unit-test `resolveSnapIndex`, `targetXForIndex`, `clampScreenSize`, etc. in isolation.

4. **Snap-back on non-change**: When the drag ends but the index doesn't change, `useCarousel` explicitly animates back to the correct resting position. This prevents the strip from drifting if the user releases mid-drag.

import onnxruntime as ort
import numpy as np

# Load model
session = ort.InferenceSession("models/tts/kokoro-v1.0.int8.onnx")

print("=== INPUTS ===")
for inp in session.get_inputs():
    print(f"Name: {inp.name}")
    print(f"Shape: {inp.shape}")
    print(f"Type: {inp.type}")
    print()

print("=== OUTPUTS ===")
for out in session.get_outputs():
    print(f"Name: {out.name}")
    print(f"Shape: {out.shape}")
    print(f"Type: {out.type}")
    print()

# Try minimal inference
print("=== TEST INFERENCE ===")
try:
    # Minimal inputs
    tokens = np.array([[0, 10, 11, 12, 0]], dtype=np.int64)
    style = np.zeros((1, 256), dtype=np.float32)
    speed = np.array([1.0], dtype=np.float32)
    
    print("Trying with: tokens, style, speed")
    outputs = session.run(None, {
        "tokens": tokens,
        "style": style,
        "speed": speed
    })
    print(f"Success! Output shape: {outputs[0].shape}")
except Exception as e:
    print(f"Error: {e}")

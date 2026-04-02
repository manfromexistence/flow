#!/usr/bin/env python3
"""
REAL Kokoro TTS ONNX Runner
Uses onnxruntime to run the actual Kokoro model
"""
import sys
import json
import numpy as np
import onnxruntime as ort
import soundfile as sf

def text_to_tokens(text):
    """Simple character-based tokenization"""
    tokens = [0]  # BOS
    
    for ch in text:
        if 'a' <= ch <= 'z':
            token = ord(ch) - ord('a') + 10
        elif 'A' <= ch <= 'Z':
            token = ord(ch) - ord('A') + 36
        elif ch == ' ':
            token = 16
        elif ch == '.':
            token = 4
        elif ch == ',':
            token = 5
        elif ch == '!':
            token = 6
        elif ch == '?':
            token = 7
        elif ch == ':':
            token = 8
        elif ch == "'":
            token = 9
        else:
            token = 16
        tokens.append(token)
    
    tokens.append(0)  # EOS
    return tokens

def main():
    if len(sys.argv) < 3:
        print(json.dumps({"error": "Usage: tts_runner.py <text> <output_wav>"}))
        sys.exit(1)
    
    text = sys.argv[1]
    output_path = sys.argv[2]
    
    try:
        print("Loading model...", file=sys.stderr)
        # Load model
        session = ort.InferenceSession(
            "models/tts/kokoro-v1.0.int8.onnx",
            providers=['CPUExecutionProvider']
        )
        print("Model loaded", file=sys.stderr)
        
        print("Loading voices...", file=sys.stderr)
        # Load voice embeddings
        with open("models/tts/voices-v1.0.bin", "rb") as f:
            voice_data = f.read()
        
        # Ensure buffer size is multiple of 4 (float32)
        voice_size = len(voice_data) // 4 * 4
        voices = np.frombuffer(voice_data[:voice_size], dtype=np.float32)
        
        # Use first 256 floats as voice embedding
        if len(voices) >= 256:
            voice_embedding = voices[:256].reshape(1, 256)
        else:
            # Pad if needed
            voice_embedding = np.zeros((1, 256), dtype=np.float32)
            voice_embedding[0, :len(voices)] = voices
        print("Voices loaded", file=sys.stderr)
        
        print("Tokenizing...", file=sys.stderr)
        # Tokenize
        tokens = text_to_tokens(text)
        input_ids = np.array([tokens], dtype=np.int64)
        speed = np.array([1.0], dtype=np.float32)
        print(f"Tokens: {len(tokens)}", file=sys.stderr)
        
        print("Running inference...", file=sys.stderr)
        # Run inference
        outputs = session.run(
            None,
            {
                "tokens": input_ids,
                "style": voice_embedding,
                "speed": speed
            }
        )
        print("Inference complete", file=sys.stderr)
        
        audio = outputs[0].flatten()
        
        # Save WAV
        sf.write(output_path, audio, 24000)
        
        print(json.dumps({
            "success": True,
            "samples": len(audio),
            "duration": len(audio) / 24000.0
        }))
        
    except Exception as e:
        print(json.dumps({"error": str(e)}))
        sys.exit(1)

if __name__ == "__main__":
    main()

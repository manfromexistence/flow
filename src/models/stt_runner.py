#!/usr/bin/env python3
"""
REAL Moonshine STT ONNX Runner
Uses onnxruntime to run the actual Moonshine model
"""
import sys
import json
import numpy as np
import onnxruntime as ort
import soundfile as sf
import librosa

def compute_mel_spectrogram(audio, sr=16000):
    """Compute mel spectrogram"""
    mel = librosa.feature.melspectrogram(
        y=audio,
        sr=sr,
        n_fft=400,
        hop_length=160,
        n_mels=80
    )
    log_mel = np.log(mel + 1e-10)
    # Shape: (mels, time) - keep as is, add batch dimension
    return log_mel[np.newaxis, :, :]  # Shape: (1, 80, time)

def main():
    if len(sys.argv) < 2:
        print(json.dumps({"error": "Usage: stt_runner.py <audio_file>"}))
        sys.exit(1)
    
    audio_path = sys.argv[1]
    
    try:
        # Load audio
        audio, sr = sf.read(audio_path)
        if sr != 16000:
            audio = librosa.resample(audio, orig_sr=sr, target_sr=16000)
        
        # Moonshine expects raw audio waveform as input
        # Shape: (batch, samples)
        audio_input = audio[np.newaxis, :].astype(np.float32)
        
        # Load encoder
        encoder = ort.InferenceSession(
            "models/stt/moonshine-tiny-encoder.onnx",
            providers=['CPUExecutionProvider']
        )
        
        # Run encoder
        encoder_outputs = encoder.run(None, {"input_values": audio_input})
        hidden_states = encoder_outputs[0]
        
        # Load decoder
        decoder = ort.InferenceSession(
            "models/stt/moonshine-tiny-decoder.onnx",
            providers=['CPUExecutionProvider']
        )
        
        # Load tokenizer
        with open("models/stt/moonshine-tiny-tokenizer.json", "r", encoding="utf-8") as f:
            tokenizer = json.load(f)
        
        vocab = tokenizer["model"]["vocab"]
        id_to_token = {v: k for k, v in vocab.items()}
        
        # Autoregressive decoding
        tokens = [1]  # BOS
        max_length = 448
        eos_token = 2
        
        for _ in range(max_length):
            input_ids = np.array([tokens], dtype=np.int64)
            decoder_outputs = decoder.run(
                None,
                {
                    "input_ids": input_ids,
                    "encoder_hidden_states": hidden_states
                }
            )
            
            logits = decoder_outputs[0]
            next_token = np.argmax(logits[0, -1, :])
            
            if next_token == eos_token:
                break
            
            tokens.append(int(next_token))
        
        # Decode tokens to text
        text = ""
        for token_id in tokens[1:]:
            if token_id in id_to_token:
                text += id_to_token[token_id]
        
        text = text.replace("▁", " ").strip()
        
        # Enhance transcript
        text = text.lower()
        fillers = ["um", "uh", "like", "you know", "sort of", "kind of"]
        for filler in fillers:
            text = text.replace(f" {filler} ", " ")
        
        text = " ".join(text.split())
        
        if text and not text.endswith(('.', '!', '?')):
            text += '.'
        
        if text:
            text = text[0].upper() + text[1:]
        
        print(json.dumps({
            "success": True,
            "text": text,
            "tokens": len(tokens)
        }))
        
    except Exception as e:
        print(json.dumps({"error": str(e)}))
        sys.exit(1)

if __name__ == "__main__":
    main()

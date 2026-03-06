Shohan@dx MINGW64 /f/edith (main)
$ cargo build --release && ./target/release/edith.exe
   Compiling edith v0.1.0 (F:\edith)
    Finished `release` profile [optimized] target(s) in 6.67s

◆ Edith AI CLI
  Professional streaming interface

Initializing llama.cpp backend...
Model loaded: Qwen 3 0.6B Q4_K_M
Model max context: 40960 tokens
Runtime context: 32768 tokens
Vocabulary: 151936 tokens
Requested max output tokens: 32768 (set EDITH_MAX_TOKENS=1..32768)
Max generation time: 60s (set EDITH_MAX_GENERATION_SECONDS=1..900)
Thinking view: ON (set EDITH_SHOW_THINKING=0 to hide)
Decoding: temp=0.75, top_p=0.9, top_k=40, min_p=0.05, repeat_penalty=1.12
Type 'exit' or 'quit' to end the session.

You › Hello, what is 2+2??  
Input tokens: 17
Reasoning › Okay, the user asked, "What is 2 + 2?" and I need to respond. Let me think about this.

First, I should check if there's any context that might imply otherwise. But since it's a basic arithmetic question, I can directly answer. Adding two twos together would be 4. However, sometimes people might make a mistake here. Maybe they expect something else? Wait, but unless there's some hidden trick or cultural reference, like in a specific math problem, I don't think so. So the straightforward answer is 4. Alternatively, maybe the user wants a more elaborate explanation. Let me confirm once again without assuming anything else.
Edith › 

The sum of 2 plus 2 is **4**. 

Let me know if you have another question! ��

◆ Performance metrics
  Model:             Qwen 3 0.6B Q4_K_M
  Input Tokens:      17
  Generated Tokens:  166
  Output Cap:        32751
  Answer Chars:      78
  Thinking Chars:    628
  Time:              4.93 sec
  Tokens/Sec:        33.66 tok/s
  Memory Delta:      188 MB
  Total Memory:      6918 MB
  CPU Usage:         50.6%

You ›

Shohan@dx MINGW64 /f/edith (main)
$ cargo build --release && ./target/release/edith.exe
   Compiling edith v0.1.0 (F:\edith)
    Finished `release` profile [optimized] target(s) in 7.29s

◆ Edith AI CLI
  Professional streaming interface

Initializing llama.cpp backend...
Model loaded: Qwen 3.5 0.8B Q4_K_M
Model max context: 262144 tokens
Runtime context: 32768 tokens
Vocabulary: 248320 tokens
Requested max output tokens: 32768 (set EDITH_MAX_TOKENS=1..32768)
Max generation time: 60s (set EDITH_MAX_GENERATION_SECONDS=1..900)
Thinking view: ON (set EDITH_SHOW_THINKING=0 to hide)
Decoding: temp=0.75, top_p=0.9, top_k=40, min_p=0.05, repeat_penalty=1.12
Type 'exit' or 'quit' to end the session.

You › Hello, what is 2+2??  
Input tokens: 17
Edith › 

$2 + 2 = 4$. ��

Shohan@dx MINGW64 /f/edith (main)
$ cargo build --release && ./target/release/edith.exe
    Finished `release` profile [optimized] target(s) in 0.19s

◆ Edith AI CLI
  Professional streaming interface

Initializing llama.cpp backend...
Model loaded: Qwen 3.5 0.8B Q4_K_M
Model max context: 262144 tokens
Runtime context: 32768 tokens
Vocabulary: 248320 tokens
Requested max output tokens: 32768 (set EDITH_MAX_TOKENS=1..32768)
Max generation time: 60s (set EDITH_MAX_GENERATION_SECONDS=1..900)
Thinking view: ON (set EDITH_SHOW_THINKING=0 to hide)
Decoding: temp=0.75, top_p=0.9, top_k=40, min_p=0.05, repeat_penalty=1.12
Type 'exit' or 'quit' to end the session.

You › Hello, what is 2+2??  
Input tokens: 17
Edith › 

$${ } + $} + $$ = **4**

The mathematical symbol for addition (+) and the number "two" (which in this context means two numbers added together) are often represented by the Roman numeral $\text{II}$ to form the expression $\text{I}+\text{II}$. This evaluates to four. If you meant a different operation or specific context, feel free to ask!

◆ Performance metrics
  Model:             Qwen 3.5 0.8B Q4_K_M
  Input Tokens:      17
  Generated Tokens:  88
  Output Cap:        32751
  Answer Chars:      343
  Thinking Chars:    0
  Time:              5.10 sec
  Tokens/Sec:        17.27 tok/s
  Memory Delta:      396 MB
  Total Memory:      4169 MB
  CPU Usage:         40.7%

You ›

Shohan@dx MINGW64 /f/edith (main)
$ cargo build --release && ./target/release/edith.exe
   Compiling edith v0.1.0 (F:\edith)
    Finished `release` profile [optimized] target(s) in 6.25s

◆ Edith AI CLI
  Professional streaming interface

Initializing llama.cpp backend...
Model loaded: Qwen 3.5 2B Q4_K_M
Model max context: 262144 tokens
Runtime context: 32768 tokens
Vocabulary: 248320 tokens
Requested max output tokens: 32768 (set EDITH_MAX_TOKENS=1..32768)
Max generation time: 60s (set EDITH_MAX_GENERATION_SECONDS=1..900)
Thinking view: ON (set EDITH_SHOW_THINKING=0 to hide)
Decoding: temp=0.75, top_p=0.9, top_k=40, min_p=0.05, repeat_penalty=1.12
Type 'exit' or 'quit' to end the session.

You › Hello, what is 2+2??  
Input tokens: 17
Edith › 

Hello! The answer to **$2 + 2 = ?$** is simply **4**.

It's a very common math problem that always equals four. Is there something specific you'd like to learn about or ask about?

◆ Performance metrics
  Model:             Qwen 3.5 2B Q4_K_M
  Input Tokens:      17
  Generated Tokens:  51
  Output Cap:        32751
  Answer Chars:      181
  Thinking Chars:    0
  Time:              5.28 sec
  Tokens/Sec:        9.66 tok/s
  Memory Delta:      696 MB
  Total Memory:      5215 MB
  CPU Usage:         40.9%

You ›

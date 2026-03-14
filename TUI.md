 Now here's the thing: our chat tui are really doing only this task but as you can see our chat tui has so many files and folders which are useless. Now editing it is a huge task so that's why I want to create another thread that copies those files and then removes and comments out other useless stuff. Only create a rust thread using the crates needed to show this feature, because these are the real features that we are interested in and all other files in the TOI are just a waste of time now.
Can you hear my word correctly? Copy these files, copy these important files. As you can see it's barely less than ten files. Copy these files and create a new crate. In that crate use all the crates that are needed to show all these features and use the latest crates of them. Create the new rust crates on this workspace. 

| Feature | File Path | Key Lines | What It Does |
|---------|-----------|-----------|--------------|
| **GGUF Model Path** | `src/ui/chat/local_llm.rs` | 14-15 | Hardcoded model file path |
| **Model Loading** | `src/ui/chat/local_llm.rs` | 73-91 | Loads GGUF into memory via llama.cpp |
| **Token Generation** | `src/ui/chat/local_llm.rs` | 217-332 | Generates text token-by-token with streaming |
| **Message List** | `src/ui/chat/components.rs` | 50-290 | Renders chat messages with markdown |
| **Arrow Key Detection** | `src/ui/chat/app_events.rs` | 216-221 | Detects left/right arrows |
| **Animation Navigation** | `src/ui/chat/app_events.rs` | 1011-1055 | Changes animation index |
| **Animation Dispatcher** | `src/ui/chat/app_render.rs` | 63-130 | Routes to correct animation |
| **Matrix Animation** | `src/ui/chat/app_render.rs` | 711-785 | Falling green characters |
| **Train Animation** | `src/ui/chat/app_render.rs` | 790-930 | Moving ASCII train |
| **Splash Screen** | `src/ui/chat/app_splash.rs` | Full file | FIGlet ASCII art |

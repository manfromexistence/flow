Awesome now you hardcoded a chat input in the bottom - now did you used rataui our mlua layout that to show that chatinput as our main chat tui is made of ratatui and also I know for a fact that yazi also uses rataui - so now as both yazi ui and our one hardcoded chat ui is showing so please give me best way to render our whole actual rataui tui there correctly - give me plan for now

Awesome now then copy our code from the main src folder and please use our main actual code instead of creating new ai slop I created the tui with care and it will be really hard if you try to recreate it, it will be ai slop so please use our actual code as its very less amount of code anyway and integrate it correctly - now handle the yazi logic correctly and keep it mind do it performantly as yazi will be the default file picker - like there will be a screen in our chat input with the yazi tui from where users can select the path of our tui workspace as our tui is a ai tui and users can also see file content and stuffs - so please create a plan to integrate out actual ui correctly in yazi

Awesome now please do it!!!

Please look at the main root src folder and and its cargo.toml and commnent out all code related to yazi integrations in our main src folder crate as we holded the integration to yazi for now

Please look at the main root src folder and and its cargo.toml and commnent out all code related to yazi integrations in our main src folder crate as we holded the integration to yazi for now

Awesome our chat input is showing now the chat input box is way huge like 3x the size it should be so please cut its height 2x and also in that space show yazi file picker layout

Now here is the main problem. Currently the yazi is using Lua and previously our SRC folder effects used a rataUI more deeply. Now when I scroll on the center of the file browser, it is updating the rainbow animation on the chat input cursor. That means the bottom animations are not being triggered on an animated time loop basis but it's showing them when we are interacting with the Lua buffer and rendering. Make sure that our chat input uses other rendering or so that the Lua top rendering doesn't affect our effects and animations and they can run smoothly by their own like how they used to run. Now don't give me any code and just tell me why our animations on the bottom are not running separately on their own. Is there any reason? Because the top is the file that we use so why is that even affecting our chat bottom animations? 
Oh got it. Now previously in the src root folder we are using our own event timer, right? Among ten how hard is it to create our custom event timer to get our custom events on the age project? Is it hard or easy to do it? 

Now remember the left panel not scrollable problem - now its being scrollbar but when left panel changes the center items should updated the left panels content so please do it currently it not updating center panel hence both cener and rigth panel based on the left panel change so please do it!!! And please do it fast, just implement the fix and don't test anything as you don't much limit so do it as fast as possible and first just out the plan to fix it without testing anything or what so ever!!!

Please make sure that the root hrc folder, as it has the message list screen in our app, also has the message list working correctly with all the features like the hrc root folder message list screen. As currenlty when I input somehing its not adding in the chat input box so please fix that too

Now please look at the root tui folder and there you can see it's a yaji rast cd live file browser process. Now we have shown our custom tui on that. Now as you can see the custom tui in our main quick users and the file browser init, now the yaji has shown class crates and dependencies. Now if you can then make it so that we can still have the yaji file browser on our tui with as few crates as possible.

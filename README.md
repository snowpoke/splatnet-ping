# Splatnet-ping
With this tool, you'll have to update your token substantially less often! (maybe even never)

Here's how you set it up:
1. Put the splatnet_ping.exe file in the same folder as woomyDX.exe
2. Press WIN+R and type "shell:startup", press enter
3. Right click, New > New Shortcut
4. Browse to the location of splatnet_ping.exe
5. Make sure that woomyDX has an active token (should show your power when you open it)
6. Restart your computer

From now on, splatnet_ping.exe will send a ping to Nintendo servers every hour that tells them that your token is still active, which makes it last way longer!

Oh, and if you ever refresh your token with woomyDX, splatnet_ping will adjust and ping that token instead - you don't need to change any settings here.

To be used in conjunction with woomyDX: https://github.com/snowpoke/XPowerControl

## About the source code
This was my first Rust project, and it was written in an evening. But hey, it works ¯\\_(ツ)_/¯

## License
MIT / Apache

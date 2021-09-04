# **NOTE**
This only works with packs that have a published server pack.
This also can break if anything within the curseforge api changes, which is semi probable.

# Info
This is a server pack downloader for minecraft CurseForge.
You can see all the arguments and what they do by using the ``--help`` parameter.

# Example Output
```
❯ ./curseforge-pack-downloader.exe --query e2e
Please pick the client pack you would like the server pack for.
1: Enigmatica2Expert-1.81.zip
2: Enigmatica2Expert-1.82.zip
3: Enigmatica2Expert-1.83.zip
3
Downloading server file `./Enigmatica2ExpertServer-1.83.zip` from `https://edge.forgecdn.net/files/3433/600/Enigmatica2ExpertServer-1.83.zip`.
Downloaded https://edge.forgecdn.net/files/3433/600/Enigmatica2ExpertServer-1.83.zip to ./Enigmatica2ExpertServer-1.83.zip
  [00:00:36] [###############################################################################################################################] 312.63MiB/312.63MiB (8.59MiB/s, 0s)
```

# Compiling
Clone the repo and then run `cargo build --release`.
Then you can just simply run that executable.
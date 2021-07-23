# Info
This is a server pack downloader for minecraft CurseForge.
You can see all the arguments and what they do by using the ``--help`` parameter.

# Example Output
```
❯ ./curseforge-pack-downloader -s e2e -m 2
Please pick the pack you want.
1: Enigmatica 2: Expert - E2E
2: Enigmatica 2: Expert Skyblock - E2ES
2
Please pick the client pack you would like the server pack for.
1: Enigmatica2ExpertSkyblock-1.36.zip
2: Enigmatica2ExpertSkyblock-1.36a.zip
1
Downloading server files `Enigmatica2ExpertSkyblock-1.36.zip` from `https://edge.forgecdn.net/files/3239/672/Enigmatica2ExpertSkyblockServer-1.36.zip`
```

# Compiling
Clone the repo and then run `cargo build --release`.
Then you can just simply run that executable.

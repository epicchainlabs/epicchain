FROM --platform=$BUILDPLATFORM mcr.microsoft.com/dotnet/sdk:8.0 AS Build

# Run this from the repository root folder
COPY src .
COPY NuGet.Config /EpicChain.CLI

WORKDIR /EpicChain.CLI
RUN dotnet restore && dotnet publish  -f net8.0 -c Release -o /app

FROM --platform=$BUILDPLATFORM mcr.microsoft.com/dotnet/aspnet:8.0 AS Final
RUN apt-get update && apt-get install -y \
  screen \
  libleveldb-dev \
  sqlite3
RUN rm -rf /var/lib/apt/lists/*

WORKDIR /EpicChain.CLI
COPY  --from=Build /app .

ENTRYPOINT ["screen","-DmS","node","dotnet","epicchain-cli.dll","-r"]

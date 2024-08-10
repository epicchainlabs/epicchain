dotnet publish -r osx-x64 --sc -c Release -o ClientApp/build-neo-node
cd ClientApp
npm install
npm run publish
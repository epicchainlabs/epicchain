# EpicChain-GUI

Welcome to the EpicChain-GUI project! Our graphical user interface for EpicChain is an integral component in the ongoing development of the EpicChain ecosystem. As we advance, we are committed to continually refining and expanding the features of EpicChain-GUI to provide an optimal user experience. We appreciate your engagement and encourage you to share your feedback and suggestions to help us enhance this tool.

## Required Tools and Dependencies for Development

To effectively contribute to and develop the EpicChain-GUI project, please ensure that you have the following tools and dependencies installed on your development machine:

1. **[Visual Studio 2019](https://visualstudio.microsoft.com/)**: This integrated development environment (IDE) is essential for the development and debugging of EpicChain-GUI. Visual Studio 2019 provides a robust platform for managing code, running tests, and debugging the application.

2. **[Visual Studio .NET Core 6.0](https://dotnet.microsoft.com/download)**: Ensure that you have .NET Core 6.0 installed, as it is the runtime environment required for executing .NET-based applications. This version of .NET Core includes the necessary libraries and tools to support the application’s backend services.

3. **[Node.js](https://nodejs.org/)**: Node.js is a JavaScript runtime essential for managing front-end dependencies and executing scripts. It provides the necessary environment for running the JavaScript tooling required by EpicChain-GUI.

## Build and Run

### Installing Front-End Dependencies

To set up the client-side application, navigate to the `epicchain-gui/epicchain-gui/ClientApp` directory within your project structure. From this directory, execute the following command to install all necessary JavaScript dependencies:

```shell
npm install
```

This command will download and configure the various libraries and modules needed for the front-end of EpicChain-GUI. These dependencies are crucial for ensuring that the graphical interface functions as intended.

### Running EpicChain-GUI

After the JavaScript dependencies have been successfully installed, you can proceed to run or debug EpicChain-GUI. To do this, open the solution file located at `./epicchain-gui/epicchain-gui.sln` using Visual Studio 2019. Press **F5** within the IDE to start the debugging process.

Visual Studio 2019 will compile the code and launch EpicChain-GUI, allowing you to interact with and test the application in a development environment. This step is vital for identifying and resolving any issues before the final release.

## Release

To prepare and publish an installation package for EpicChain-GUI, you will need to execute the release script. This script is located in the `epicchain-gui/epicchain-gui` directory. Use the following command to run the script:

```shell
./publish.sh
```

The `publish.sh` script automates the process of packaging the application for distribution. Upon successful execution, the installation package will be generated and placed in the default output directory. For EpicChain-GUI, the default directory is `epicchain-gui/epicchain-gui/ClientApp/build-electron`.

Within this directory, you will find the final installation files necessary for deploying EpicChain-GUI to end users. These files can be used to create an installer or distribute the application directly.

## Contribution and Feedback

Your involvement is crucial to the success of EpicChain-GUI. We welcome all forms of feedback, whether it be suggestions for new features, reports of issues, or general comments on usability. To contribute or report issues, please visit our [issues page](https://github.com/epicchainlabs/epicchain-private-network/issues). When creating a new issue, provide as much detail as possible, including code snippets, steps to reproduce, and any relevant context. This will help us address your concerns more effectively.

Thank you for being a part of the EpicChain community and for contributing to the development of EpicChain-GUI!

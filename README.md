# Azure Security Assessment Tool

Welcome to the Azure Security Tool project! This is a Azure Security Assessment tool developed with the Rust programming language. It can be used to assess the security of cloud-based applications, services, and infrastructure. It aims to provide a fast and efficient way to identify potential vulnerabilities and security risks in Azure infrastructure and resources.

I coded this program when I first started writing Rust in 2021. Therefore, there may be many unnecessary and incorrect uses. This code has also been made into a SaaS application. If anyone is interested, they can reach me via my social media accounts and get more information about this SaaS application.

The tool provides an overview of the security posture of a cloud environment, as well as reports of areas of risk. The report includes a analysis of the security of the underlying infrastructure, as well as the security of the application layer and services. You can also customize the rules.

The tool is designed to be user friendly and intuitive. It can be run from the command line.

## Requirements
* Rust version 1.50 or higher
* Microsoft Azure account
* Bash Terminal
* base64 Command

## Features
* Scan Azure resources for common security misconfigurations and best practice violations.
* Output results in a report format, including recommendations for remediation.
* Necessary performance improvements have been made.

## Installation
1. Clone this repository:
```
https://github.com/kuzeyardabulut/azure-assessment.git
```

2. Change into the project directory:
```
cd azure-assessment
```

3. Build and run the project:
```
cargo run --release
```
If you have any questions or feedback, please don't hesitate to contact us.

## Usage
To use this tool, you will need to provide your Azure account credentials. These can be passed as command-line arguments or through a configuration file.

The tool automatically scans your infrastructure so you don't need to do anything.

## Scan Result
It shows the results of the scans in an excel file as below.
![image](https://github.com/kuzeyardabulut/azure-assessment/assets/54737933/7bdcf28d-b422-4a2b-9b19-af5b30c2a3f4)

## Customization
You can customize the behavior of this tool by modifying the config.json file in the repository. This file allows you to add or change the rules for this project.

```rust
{
      "Part" :"AppService", //Category of rule
      "num_req": "2", //Number of steps of the rule
      "Title": "Enable App Service Authentication\n", //Title of the rule
      "Descrip": "Ensure that App Service Authentication feature is enabled for Microsoft Azure App Service to add an extra layer of security to the authentication process implemented by your web applications.\n", //Description of rule
      "Command":"az", //Command of rule
      "Args": "webapp list --query [*].id", 
      "Second_Args": "webapp auth show --ids",
      "Second_To_Second_Args": "--query enabled", //Arguments of the rule's command
      "Problem": "equal", //For the query operation in the data analysis step
      "Type": "stdout", // The file descriptor is required to analyze the response returned from the query
      "Valid":"false" // The final value required for the analysis of the returned value from the query
}
```

## Contributing
We welcome contributions to this project! If you have an idea for a new feature or a bug fix, please open an issue.

## License
This project is licensed under the [GNU License](LICENSE).




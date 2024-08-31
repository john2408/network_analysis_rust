# Movie Dataset Network Analysis

Network analysis of the IMDB movie dataset. The "Top 1000 IMDb Movies Dataset" is a comprehensive collection that presents the most celebrated and beloved movies, as rated and ranked by IMDb users. 

[Link to the data](https://www.kaggle.com/datasets/inductiveanks/top-1000-imdb-movies-dataset)

The goal of this project is to calculate th most influencial people from all movies available using the columns Director and Stars 1-4. The code in this repo implements an undirected graph using petgraph and rustworkx and calculates the eigenvector centrality of the top 5 people. 

Two approaches are implemented: 
- Self-made Eigenvector centrality function. 
- Calculation of Eigenvector centrality using Rustworkx. 

This code was created using DevContainers, that means that all the dependencies and packages are directly handle by the devcontainer.json config. 

Prerequisites:
- Docker Desktop
- VSCode

In order to run the code, download VsCode and install the Devcontainers extension. Then go to "View", selected "Command Palette", and select:

 ```bash
 Dev Containers: Reopen in Container ...
 ```

After that the latest Rust version will be installed, and the code can be run using: 

 ```bash
 cargo build
 cargo run
 ```

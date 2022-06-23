setting up a python environment for the project
`python3 -m venv .env`

sourcing the local environment
`source .env/bin/activate`

installing maturin to integrate rust libraries with python
`pip3 install maturin`

initializing the rust project
`maturin init`

building the rust library for python
`maturin develop`

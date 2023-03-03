from setuptools import setup

with open("requirements.txt") as f:  
    requirements = f.read().splitlines()

setup(
    name="ai_tools",
    version="0.1",
    packages=["ai_tools"],
    install_requires=requirements
)

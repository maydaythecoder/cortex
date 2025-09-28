"""
Setup script for the Cortex programming language.
"""

from setuptools import setup, find_packages
import os

# Read the README file
def read_readme():
    readme_path = os.path.join(os.path.dirname(__file__), 'README.MD')
    if os.path.exists(readme_path):
        with open(readme_path, 'r', encoding='utf-8') as f:
            return f.read()
    return "Cortex: A compiled programming language for AI and machine learning"

# Read requirements
def read_requirements():
    requirements_path = os.path.join(os.path.dirname(__file__), 'requirements.txt')
    if os.path.exists(requirements_path):
        with open(requirements_path, 'r', encoding='utf-8') as f:
            return [line.strip() for line in f if line.strip() and not line.startswith('#')]
    return []

setup(
    name="cortex-lang",
    version="0.1.0",
    author="Cortex Team",
    author_email="team@cortex-lang.org",
    description="A compiled programming language designed for AI and machine learning",
    long_description=read_readme(),
    long_description_content_type="text/markdown",
    url="https://github.com/yourname/cortex",
    project_urls={
        "Bug Tracker": "https://github.com/yourname/cortex/issues",
        "Documentation": "https://cortex-lang.readthedocs.io/",
        "Source Code": "https://github.com/yourname/cortex",
    },
    packages=find_packages(),
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Intended Audience :: Science/Research",
        "Topic :: Scientific/Engineering :: Artificial Intelligence",
        "Topic :: Software Development :: Compilers",
        "Topic :: Software Development :: Libraries :: Python Modules",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: C",
        "Operating System :: OS Independent",
    ],
    python_requires=">=3.8",
    install_requires=read_requirements(),
    extras_require={
        "dev": [
            "pytest>=6.2.0",
            "pytest-cov>=2.12.0",
            "pytest-benchmark>=3.4.0",
            "black>=21.0.0",
            "flake8>=3.9.0",
            "mypy>=0.910",
        ],
        "docs": [
            "sphinx>=4.0.0",
            "sphinx-rtd-theme>=0.5.0",
        ],
        "gpu": [
            "cupy>=9.0.0",
            "pycuda>=2021.1",
        ],
        "viz": [
            "matplotlib>=3.4.0",
            "seaborn>=0.11.0",
            "plotly>=5.0.0",
        ],
        "jupyter": [
            "jupyter>=1.0.0",
            "ipykernel>=6.0.0",
        ],
    },
    entry_points={
        "console_scripts": [
            "cortexc=cortex.compiler.main:main",
            "cortex-run=cortex.runtime.main:main",
        ],
    },
    include_package_data=True,
    package_data={
        "cortex": [
            "runtime/include/*.h",
            "runtime/src/*.c",
            "runtime/Makefile",
            "examples/*.ctx",
            "docs/*.md",
        ],
    },
    zip_safe=False,
    keywords="programming-language compiler ai machine-learning tensor numerical-computing",
)

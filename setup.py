from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="pybookmarks",
    version="1.0",
    rust_extensions=[
      RustExtension("pybookmarks.pybookmarks", binding=Binding.PyO3)
    ],
    packages=["pybookmarks"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)

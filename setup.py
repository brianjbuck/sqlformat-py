import sys

from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="sqlformat",
    version="1.0.0",
    rust_extensions=[RustExtension("sqlformat", binding=Binding.PyO3)],
    # Rust extensions are not zip safe
    zip_safe=False,
    long_description="Formats SQL queries to a more human readable style.",
)

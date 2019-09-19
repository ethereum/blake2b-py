"""
Functions for calculating blake2b hashes.
"""
from .blake2 import extract_blake2b_parameters, blake2b_compress

__all__ = ["extract_blake2b_parameters", "blake2b_compress"]

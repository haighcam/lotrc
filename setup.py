#!/usr/bin/env python

from distutils.core import setup

setup(
    name='lotrc',
    version='0.1',
    install_requires=["numpy", "lupa"],
    packages=['lotrc'],
    package_dir = {'lotrc': '.'}
)
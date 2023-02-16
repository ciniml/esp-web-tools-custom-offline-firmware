# ESP Web Tools for Custom offline firmware.

## Overview

This is a customized version of [ESP Web Tools](https://esphome.github.io/esp-web-tools/) which supports reading firmware manifest from local storage.

## Changes

The manifest JSON can contain BASE64 encoded firmware binary as `data` field instead of specifying the source path of the binary as `path` field.

And the `manifest` attribute of the `esp-web-install-button` can hold a manifest JSON object itself instead of the manifest JSON URL.

It enables using a manifest JSON uploaded as a form data to write a firmware.

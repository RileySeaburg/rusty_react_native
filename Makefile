# Copyright (c) 2022 Evolving Software Corporation
# 
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

ARCHS_IOS = i386-apple-ios x86_64-apple-ios armv7-apple-ios armv7s-apple-ios arm64-apple-ios
ARCHS_ANDROID = i686-linux-android x86_64-linux-android armv7-linux-androideabi armv8-linux-androideabi i686-linux-android
# Not sure what this is below
LIB=libsigner.a

all: ios android

ios: $(LIB)

android: $(ARCHS_ANDROID)
		sh copy_android.sh
# Not sure what this means
.PHONY: $(ARCHS_IOS)
$(ARCHS_IOS): %:
		cargo build --target $@ --release

.PHONY: $(ARCHS_ANDROID)
$(ARCHS_ANDROID): %:
		cargo build --target $@ --release

$(LIB): $(ARCHS_IOS)
		lipo -create -output $@ $(foreach arch, $(ARCHS_IOS), $(wildcard target/$(arch)/release/$(LIB)))
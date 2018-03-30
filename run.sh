if ./build.sh; then
	echo ""
	echo ">>> Running bootimage.bin in qemu-system-x86_64"
	echo ""
	qemu-system-x86_64 -drive format=raw,file=bootimage.bin -serial stdio
else
	echo ""
	echo ">>> Run cancelled on build fail"
fi

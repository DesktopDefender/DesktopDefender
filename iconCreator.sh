# This script creates the .icns and .ico icons for macOS and Windows respectively.


#macOS
# create icons for each size needed by iconutil
sips -z 16 16 ./src-tauri/icons/DD1024x1024.png --out ./src-tauri/icons/DD.iconset/icon_16x16.png
sips -z 32 32 ./src-tauri/icons/DD1024x1024.png --out ./src-tauri/icons/DD.iconset/icon_16x16@2x.png
sips -z 32 32 ./src-tauri/icons/DD1024x1024.png --out ./src-tauri/icons/DD.iconset/icon_32x32.png
sips -z 64 64 ./src-tauri/icons/DD1024x1024.png --out ./src-tauri/icons/DD.iconset/icon_32x32@2x.png
sips -z 64 64 ./src-tauri/icons/DD1024x1024.png --out ./src-tauri/icons/DD.iconset/icon_64x64.png
sips -z 128 128 ./src-tauri/icons/DD1024x1024.png --out ./src-tauri/icons/DD.iconset/icon_64x64@2x.png
sips -z 128 128 ./src-tauri/icons/DD1024x1024.png --out ./src-tauri/icons/DD.iconset/icon_128x128.png
sips -z 256 256 ./src-tauri/icons/DD1024x1024.png --out ./src-tauri/icons/DD.iconset/icon_128x128@2x.png
sips -z 256 256 ./src-tauri/icons/DD1024x1024.png --out ./src-tauri/icons/DD.iconset/icon_256x256.png
sips -z 512 512 ./src-tauri/icons/DD1024x1024.png --out ./src-tauri/icons/DD.iconset/icon_256x256@2x.png
sips -z 512 512 ./src-tauri/icons/DD1024x1024.png --out ./src-tauri/icons/DD.iconset/icon_512x512.png
sips -z 1024 1024 ./src-tauri/icons/DD1024x1024.png --out ./src-tauri/icons/DD.iconset/icon_512x512@2x.png


# using the icons created above,
# bundle them in a macOS .icns format
iconutil -c icns ./src-tauri/icons/DD.iconset

# copy some standard size formats to the icons/ folder for non-mac and -windows OS's.
cp ./src-tauri/icons/DD.iconset/icon_32x32.png ./src-tauri/icons/DD32x32.png
cp ./src-tauri/icons/DD.iconset/icon_128x128.png ./src-tauri/icons/DD128x128.png
cp ./src-tauri/icons/DD.iconset/icon_128x128@2x.png ./src-tauri/icons/DD128x128@2x.png

# now create Store Logo
sips -z 50 50 ./src-tauri/icons/DD1024x1024.png --out ./src-tauri/icons/StoreLogo.png
# and icon.png
sips -z 512 512 ./src-tauri/icons/DD1024x1024.png --out ./src-tauri/icons/icon.png

#windows
#TODO
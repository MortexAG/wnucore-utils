copy target\release\cat.exe wnu-release\
copy target\release\grep.exe wnu-release\
copy target\release\head.exe wnu-release\
copy target\release\tail.exe wnu-release\
copy target\release\lsw.exe wnu-release\
copy target\release\rm.exe wnu-release\
copy target\release\touch.exe wnu-release\
copy target\release\wnu.exe wnu-release\
powershell Compress-Archive -Path wnu-release\* -DestinationPath wnu-windows.zip
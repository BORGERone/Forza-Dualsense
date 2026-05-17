!macro customInstall
    ; Copy backend files to AppData
    SetOutPath "$LOCALAPPDATA\Forza DualSense\backend"
    File /r "${__FILEDIR__}backend\*"
!macroend

!macro customUninstall
    ; Remove backend files from AppData
    RMDir /r "$LOCALAPPDATA\Forza DualSense"
!macroend

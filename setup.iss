[Setup]
AppId={{5433C926-9B0D-44A6-9E70-9F4F9FC3C9D2}}
AppName=LogBee
AppVersion=0.3.0
AppPublisher=Masotyu
DefaultDirName={autopf}\LogBee
DefaultGroupName=LogBee
AllowNoIcons=yes
; インストーラーの出力先
OutputDir=target\installer
OutputBaseFilename=LogBee-setup
Compression=lzma
SolidCompression=yes
WizardStyle=modern
ChangesEnvironment=yes
; システム環境変数を書き換えるため管理者権限を求める
PrivilegesRequired=admin

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "japanese"; MessagesFile: "compiler:Languages\Japanese.isl"

[Files]
; ビルド済みのバイナリを指定
Source: "target\release\LogBee.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "README.md"; DestDir: "{app}"; Flags: ignoreversion

[Registry]
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; \
    ValueType: expandsz; \
    ValueName: "Path"; \
    ValueData: "{olddata};{app}"; \
    Flags: preservestringtype

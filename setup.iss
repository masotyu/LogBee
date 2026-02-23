[Setup]
AppId={{5433C926-9B0D-44A6-9E70-9F4F9FC3C9D2}}
AppName=LogBee
AppVersion=0.1.0
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

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "japanese"; MessagesFile: "compiler:Languages\Japanese.isl"

[Files]
; ビルド済みのバイナリを指定
Source: "target\release\LogBee.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "README.md"; DestDir: "{app}"; Flags: ignoreversion

[Registry]
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"; Flags: preservestringtype

[Code]
procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  Path: string;
  AppPath: string;
begin
  if CurUninstallStep = usPostUninstall then
  begin
    // レジストリから現在の Path を取得
    if RegQueryStringValue(HKEY_LOCAL_MACHINE, 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment', 'Path', Path) then
    begin
      AppPath := ExpandConstant('{app}');

      // セミコロン付きのパスを検索して削除
      StringChangeEx(Path, ';' + AppPath, '', True);
      // 先頭にいた場合のパターンも削除
      StringChangeEx(Path, AppPath + ';', '', True);
      // 万が一単体だった場合も削除
      StringChangeEx(Path, AppPath, '', True);

      // 修正した Path をレジストリに書き戻す
      RegWriteExpandStringValue(HKEY_LOCAL_MACHINE, 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment', 'Path', Path);
    end;
  end;
end;
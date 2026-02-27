#ifndef MyAppVersion
  #define MyAppVersion "0.0.0" ; デフォルト値（直接コンパイルした時用）
#endif

[Setup]
AppId={{5433C926-9B0D-44A6-9E70-9F4F9FC3C9D2}}
AppName=LogBee
AppVersion={#MyAppVersion}
AppPublisher=Masotyu
DefaultDirName={autopf}\LogBee
DefaultGroupName=LogBee
AllowNoIcons=yes
; インストーラーの出力先
OutputDir=..\..\target\installer
OutputBaseFilename=LogBee-Setup-{#MyAppVersion}
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
Source: "..\..\target\release\LogBee.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\README.md"; DestDir: "{app}"; Flags: ignoreversion

[Registry]
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; \
    ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"; \
    Check: NeedsAddPath(ExpandConstant('{app}'))

[Code]
function NeedsAddPath(Param: string): boolean;
var
  OrigPath: string;
  SearchPath, SearchParam: string;
begin
  // レジストリから現在のPATHを取得
  if not RegQueryStringValue(HKEY_LOCAL_MACHINE,
    'SYSTEM\CurrentControlSet\Control\Session Manager\Environment',
    'Path', OrigPath) then
  begin
    Log('PATH variable not found in registry. Proceeding to add.');
    Result := True;
    exit;
  end;

  // 1. 正規化：大文字に統一
  SearchParam := UpperCase(Param);
  SearchPath := UpperCase(OrigPath);

  // 2. 正規化：末尾のバックスラッシュを削除（存在する場合）
  if (Length(SearchParam) > 0) and (SearchParam[Length(SearchParam)] = '\') then
    SetLength(SearchParam, Length(SearchParam) - 1);

  // デバッグ情報の出力
  Log('Checking PATH addition:');
  Log('  - Installing Path: ' + SearchParam);

  // 3. セミコロンで囲って厳密に検索
  // ※ SearchPath側も前後にセミコロンを付加して比較
  if Pos(';' + SearchParam + ';', ';' + SearchPath + ';') = 0 then
  begin
    Log('  - Result: Path NOT found. Adding to registry.');
    Result := True;
  end
  else
  begin
    Log('  - Result: Path ALREADY exists. Skipping.');
    Result := False;
  end;
end;
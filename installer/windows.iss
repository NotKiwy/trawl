#define AppName    "trawl"
#define AppVersion "0.1.0"
#define AppURL     "https://github.com/NotKiwy/trawl"

[Setup]
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher=NotKiwy
AppPublisherURL={#AppURL}
AppSupportURL={#AppURL}/issues
AppUpdatesURL={#AppURL}/releases
DefaultDirName={autopf}\{#AppName}
DisableProgramGroupPage=yes
OutputDir=..\dist
OutputBaseFilename=trawl-windows-x86_64-setup
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
UninstallDisplayName={#AppName}
UninstallDisplayIcon={app}\trawl.exe
MinVersion=10.0

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "addtopath"; Description: "Add {#AppName} to PATH (recommended)"; GroupDescription: "System integration:"

[Files]
Source: "trawl.exe"; DestDir: "{app}"; Flags: ignoreversion

[Registry]
Root: HKCU; Subkey: "Environment"; ValueType: expandsz; ValueName: "Path"; \
  ValueData: "{olddata};{app}"; \
  Tasks: addtopath; \
  Check: __NeedsAddPath(ExpandConstant('{app}'))

[Code]
function __NeedsAddPath(const _NewPath: string): Boolean;
var
  _ExistingPath: string;
begin
  if not RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', _ExistingPath) then
  begin
    Result := True;
    Exit;
  end;
  Result := Pos(';' + Uppercase(_NewPath) + ';', ';' + Uppercase(_ExistingPath) + ';') = 0;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  _ExistingPath, _AppDir, _SearchStr, _UpperPath, _UpperSearch: string;
  _Pos: Integer;
begin
  if CurUninstallStep <> usPostUninstall then Exit;

  _AppDir := ExpandConstant('{app}');
  if not RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', _ExistingPath) then Exit;

  _UpperPath   := ';' + Uppercase(_ExistingPath) + ';';
  _SearchStr   := ';' + Uppercase(_AppDir) + ';';
  _Pos         := Pos(_SearchStr, _UpperPath);
  if _Pos = 0 then Exit;

  Delete(_ExistingPath, _Pos, Length(_AppDir) + 1);
  RegWriteStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', _ExistingPath);
end;

program TuringProject;

{$R *.dres}

uses
  System.StartUpCopy,
  FMX.Forms,
  TuringUnit in 'TuringUnit.pas' {TuringWindow},
  Turing.Classes in 'lib\Turing.Classes.pas',
  Turing.Utils in 'lib\Turing.Utils.pas';

{$R *.res}

begin
  Application.Initialize;
  Application.CreateForm(TTuringWindow, TuringWindow);
  Application.Run;
end.

program TuringProject;

uses
  System.StartUpCopy,
  FMX.Forms,
  TuringUnit in 'TuringUnit.pas' {TuringWindow};

{$R *.res}

begin
  Application.Initialize;
  Application.CreateForm(TTuringWindow, TuringWindow);
  Application.Run;
end.

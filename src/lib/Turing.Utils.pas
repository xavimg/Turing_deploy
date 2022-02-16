unit Turing.Utils;

interface

uses System.Types, System.UITypes, FMX.Graphics;

function GetRandomColor(const Alpha : Integer) : TAlphaColor;

function CreateStarBitmap(const Density, Width, Height : Integer; Radius : Integer = 5; Luminosity : Integer = 100) : TBitmap;

implementation

function GetRandomColor(const Alpha : Integer) : TAlphaColor;
begin
  result := Alpha + Random($FFFFFF);
end;

function CreateStarBitmap(const Density, Width, Height : Integer; Radius : Integer = 5; Luminosity : Integer = 100) : TBitmap;
begin
  Result := TBitmap.Create(Width, Height);
  with Result do
  begin
    Canvas.BeginScene();
    Canvas.FillRect(TRectF.Create(0, 0, Width, Height), 255, TStrokeBrush.Create(TBrushKind.Solid, TAlphaColorRec.Black));
    for var i := 0 to Density do
    begin
      var p1 := random(Width);
      var p2 := random(Height);
      var sz := random(Radius);
      var lu := random(Luminosity) / 100;
      Canvas.FillRect(TRectF.Create(p1, p2, p1+sz, p2+sz), lu, TStrokeBrush.Create(TBrushKind.Solid, TAlphaColorRec.White));
    end;
    Canvas.EndScene();
  end;
end;

end.

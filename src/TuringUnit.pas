unit TuringUnit;

interface

uses
  System.SysUtils, System.Types, System.UITypes, System.Classes, System.Variants,
  FMX.Types, FMX.Controls, FMX.Forms, FMX.Graphics, FMX.Dialogs, FMX.Viewport3D,
  System.Math.Vectors, FMX.Layers3D, FMX.Controls3D, FMX.Objects3D,
  FMX.Controls.Presentation, FMX.StdCtrls, FMX.Objects, FMX.Styles.Objects,
  FMX.Ani, FMX.MaterialSources, System.Generics.Collections, FMX.Types3D, Turing.Utils,
  FMX.Media, FMX.TabControl, Turing.Classes;

type
  TTuringWindow = class(TForm)
    MainViewport: TViewport3D;
      MenuLayer: TLayer3D;
        MenuLayerBackground: TImage;
          MenuTitle: TLabel;
          MenuBottom: TRectangle;
            PlayButton: TRectangle;
            PlayButtonLabel: TLabel;
      MenuPlanet: TSphere;
        MenuPlanetMaterial: TLightMaterialSource;
          MenuPlanetMaterialAnimation: TBitmapAnimation;
        MenuPlanetAnimation1: TFloatAnimation;
        MenuPlanetAnimation2: TFloatAnimation;
        MenuPlanetAnimation3: TFloatAnimation;
      MenuLight: TLight;
        MenuLightAnimation: TColorAnimation;
    MusicMediaPlayer: TMediaPlayer;
    HoverMediaPlayer: TMediaPlayer;
    TabControl: TTabControl;
    Login: TTabItem;
    Settings: TTabItem;
    Menu: TTabItem;
    Game: TTabItem;
    procedure FormCreate(Sender: TObject);
    procedure FormResize(Sender: TObject);
    procedure FormShow(Sender: TObject);
    procedure ButtonHover(Sender : TRectangle);
    procedure ButtonUnhover(Sender : TRectangle);
    procedure MenuPlanetMaterialAnimationFinish(Sender: TObject);
    procedure MenuLightAnimationFinish(Sender: TObject);
    procedure PlayButtonMouseEnter(Sender: TObject);
    procedure PlayButtonMouseLeave(Sender: TObject);
    procedure PlayButtonClick(Sender: TObject);

  public
    AssetPath : String;
    ResourceNames : TArray<String>;
    Resources : TDictionary<String, TResourceStream>;

    Turing : TTuringManager;

    procedure NiceErrorHandle(const Text : string);
    procedure LoadResources();
    function GetRandomPlanetBitmap() : TBitmap;
  end;

  const AccentColor : Cardinal = $FFF58A42;

  const PlanetResourceNames : TArray<String> = [
      //'Earth_day', // Easter Eggs
      //'Earth_night', // Easter Eggs
      'Mercury',
      'Venus',
      'Venus_atm',
      'Jupiter',
      'Uranus',
      'Neptune'
  ];

var
  TuringWindow: TTuringWindow;

implementation

{$R *.fmx}

procedure TTuringWindow.NiceErrorHandle(const Text: string);
begin
  showmessage(Text);
end;

procedure TTuringWindow.LoadResources();
begin
  ResourceNames := PlanetResourceNames;
  Resources := TDictionary<String, TResourceStream>.Create();
  try
    for var ResourceName in ResourceNames do
    begin
      var ResourceStream : TResourceStream;
      ResourceStream  := TResourceStream.Create(HInstance, ResourceName, RT_RCDATA);
      Resources.Add(ResourceName, ResourceStream);
    end;
  except on E: Exception do
    NiceErrorHandle(E.Message);  
  end;
end;

function TTuringWindow.GetRandomPlanetBitmap() : TBitmap;
begin
  var i : Integer := Random(length(PlanetResourceNames)-1);
  var p : String := PlanetResourceNames[i];
  var r : TResourceStream := Resources.Items[p];
  result := TBitmap.CreateFromStream(r);
end;

procedure TTuringWindow.MenuPlanetMaterialAnimationFinish(Sender: TObject);
begin
  try
    var b : TBitmap := GetRandomPlanetBitmap();
    with Sender as TBitmapAnimation do
    begin
      StartValue := StopValue;
      StopValue := b;
      Start;
    end;
    b.Free;
  except on E: Exception do
    NiceErrorHandle(E.Message);
  end;

end;

procedure TTuringWindow.MenuLightAnimationFinish(Sender: TObject);
begin
  with (Sender as TColorAnimation) do
  begin
    StopValue := GetRandomColor($FF);
    Start;
  end;
end;

procedure TTuringWindow.ButtonHover(Sender: TRectangle);
begin
  Sender.Fill.Color := AccentColor;
  HoverMediaPlayer.Stop;
  HoverMediaPlayer.CurrentTime := 0;
  HoverMediaPlayer.Play;
end;

procedure TTuringWindow.ButtonUnhover(Sender: TRectangle);
begin
  Sender.Fill.Color := $00000000;
end;

procedure TTuringWindow.FormCreate(Sender: TObject);
begin
  LoadResources();
  AssetPath := ExpandFileName(GetCurrentDir + '\..\..\dat');
end;

procedure TTuringWindow.FormResize(Sender: TObject);
begin
  MenuLayer.Width := (Sender as TForm).Width;
  MenuLayer.Height := (Sender as TForm).Height;
  MenuLayerBackground.Bitmap := CreateStarBitmap(100, Round(Screen.Width), Round(Screen.Height));
end;

procedure TTuringWindow.FormShow(Sender: TObject);
begin
  // Init Menu
  MenuPlanetMaterial.Texture := GetRandomPlanetBitmap();
  MusicMediaPlayer.FileName := AssetPath + '\sound\music\aerial.mp3';
  MusicMediaPlayer.Play;
  HoverMediaPlayer.FileName := AssetPath + '\sound\sfx\click.wav';
end;

procedure TTuringWindow.PlayButtonMouseEnter(Sender: TObject);
begin
  ButtonHover(Sender as TRectangle);
end;

procedure TTuringWindow.PlayButtonMouseLeave(Sender: TObject);
begin
  ButtonUnhover(Sender as TRectangle);
end;

procedure TTuringWindow.PlayButtonClick(Sender: TObject);
begin
  //
  Turing := TTuringManager.Create('', '', '', AssetPath, Game);
end;

end.

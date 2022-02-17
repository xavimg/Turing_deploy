unit TuringUnit;

interface

uses
  System.SysUtils, System.Types, System.UITypes, System.Classes, System.Variants, System.Math,
  FMX.Types, FMX.Controls, FMX.Forms, FMX.Graphics, FMX.Dialogs, FMX.Viewport3D,
  System.Math.Vectors, FMX.Layers3D, FMX.Controls3D, FMX.Objects3D,
  FMX.Controls.Presentation, FMX.StdCtrls, FMX.Objects, FMX.Styles.Objects,
  FMX.Ani, FMX.MaterialSources, System.Generics.Collections, FMX.Types3D, Turing.Utils,
  FMX.Media, FMX.TabControl, Turing.Classes, FMX.Edit, FMX.Memo.Types,
  FMX.ScrollBox, FMX.Memo, FMX.Effects, FMX.Filter.Effects;

type
  TTuringWindow = class(TForm)
    MusicMediaPlayer: TMediaPlayer;
    HoverMediaPlayer: TMediaPlayer;
    TabControl: TTabControl;
      Login: TTabItem;
      Settings: TTabItem;
      Menu: TTabItem;
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
      Game: TTabItem;
    Rectangle1: TRectangle;
    Rectangle2: TRectangle;
    Rectangle3: TRectangle;
    Edit1: TEdit;
    StyleBook: TStyleBook;
    Edit2: TEdit;
    CheckBox1: TCheckBox;
    Rectangle4: TRectangle;
    Label1: TLabel;
    Rectangle5: TRectangle;
    Label2: TLabel;
    Viewport: TViewport3D;
    Rectangle6: TRectangle;
    Rectangle7: TRectangle;
    Label3: TLabel;
    Edit3: TEdit;
    Rectangle8: TRectangle;
    Memo1: TMemo;
    Rectangle9: TRectangle;
    Label4: TLabel;
    Rectangle10: TRectangle;
    Label5: TLabel;
    Rectangle11: TRectangle;
    Label6: TLabel;
    Rectangle12: TRectangle;
    Label7: TLabel;
    Rectangle13: TRectangle;
    Label8: TLabel;
    Camera: TCamera;
    Star: TSphere;
    TextureMaterialSource1: TTextureMaterialSource;
    Player: TModel3D;
    Image1: TImage;
    LightMaterialSource1: TLightMaterialSource;
    Light1: TLight;
    FloatAnimation1: TFloatAnimation;
    Plane1: TPlane;
    TextureMaterialSource2: TTextureMaterialSource;
    FloatAnimationX: TFloatAnimation;
    FloatAnimationY: TFloatAnimation;
    Image2: TImage;
    Timer1: TTimer;
    Sphere1: TSphere;
    FloatAnimation2: TFloatAnimation;
    LightMaterialSource2: TLightMaterialSource;
    Layer3D1: TLayer3D;
    Rectangle14: TRectangle;
    Label9: TLabel;
    Rectangle15: TRectangle;
    Label10: TLabel;
    Sphere2: TSphere;
    ColorMaterialSource1: TColorMaterialSource;
    FloatAnimation3: TFloatAnimation;
    Cube1: TCube;
    LightMaterialSource3: TLightMaterialSource;
    FloatAnimation4: TFloatAnimation;
    FloatAnimation5: TFloatAnimation;
    Cube2: TCube;
    Cube3: TCube;
    Cube4: TCube;
    Cube5: TCube;
    Cube6: TCube;
    procedure FormCreate(Sender: TObject);
    procedure FormResize(Sender: TObject);
    procedure FormShow(Sender: TObject);
    procedure ButtonHover(Sender : TRectangle);
    procedure ButtonUnhover(Sender : TRectangle);
    procedure MenuPlanetMaterialAnimationFinish(Sender: TObject);
    procedure MenuLightAnimationFinish(Sender: TObject);
    procedure PlayButtonMouseEnter(Sender: TObject);
    procedure PlayButtonMouseLeave(Sender: TObject);
    procedure Rectangle4MouseEnter(Sender: TObject);
    procedure Rectangle4MouseLeave(Sender: TObject);
    procedure Rectangle4Click(Sender: TObject);
    procedure PlayButtonClick(Sender: TObject);
    procedure Edit3KeyDown(Sender: TObject; var Key: Word; var KeyChar: Char;
      Shift: TShiftState);
    procedure Plane1MouseDown(Sender: TObject; Button: TMouseButton;
      Shift: TShiftState; X, Y: Single; RayPos, RayDir: TVector3D);
    procedure Plane1MouseUp(Sender: TObject; Button: TMouseButton;
      Shift: TShiftState; X, Y: Single; RayPos, RayDir: TVector3D);
    procedure FormKeyDown(Sender: TObject; var Key: Word; var KeyChar: Char;
      Shift: TShiftState);
    procedure Sphere1Click(Sender: TObject);
    procedure Rectangle15Click(Sender: TObject);

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
  buttondown : boolean;
  speedx, speedy: double;

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
    with Sender as TBitmapAnimation do
    begin
      if StopValue.IsEmpty then
        StartValue := MenuPlanetMaterial.Texture
      else
        StartValue := StopValue;
      StopValue := GetRandomPlanetBitmap;
      Start;
    end;
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

procedure TTuringWindow.Edit3KeyDown(Sender: TObject; var Key: Word;
  var KeyChar: Char; Shift: TShiftState);
begin
  if Key = 13 then
    begin
    Memo1.Lines.Add(
      'Anonymous Player (Solar System): ' + Edit3.Text
    );
    Edit3.Text := '';
  end;

end;

procedure TTuringWindow.FormCreate(Sender: TObject);
begin
  LoadResources();
  AssetPath := ExpandFileName(GetCurrentDir + '\..\..\dat');
end;

procedure TTuringWindow.FormKeyDown(Sender: TObject; var Key: Word;
  var KeyChar: Char; Shift: TShiftState);
begin
var speed : double := 0.1;
  case KeyChar of
    'w':
    begin
      Player.Position.Y := Player.Position.Y - speed;
      Player.RotationAngle.Y := 90;
    end;
    's':
    begin
      Player.Position.Y := Player.Position.Y + speed;
      Player.RotationAngle.Y := 270;
    end;
    'd':
    begin
      Player.Position.X := Player.Position.X + speed;
      Player.RotationAngle.Y := 180;
    end;
    'a':
    begin
      Player.Position.X := Player.Position.X - speed;
      Player.RotationAngle.Y := 0;
    end;
  end;
  Camera.Position.X := Player.Position.X;
  Camera.Position.Y := Player.Position.Y;
  Plane1.Position.X := Player.Position.X;
  Plane1.Position.Y := Player.Position.Y;
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
  MenuPlanetMaterialAnimation.StartValue := MenuPlanetMaterial.Texture;
  MenuPlanetMaterialAnimation.Start;
  MusicMediaPlayer.FileName := AssetPath + '/sound/music/aerial.mp3';
  MusicMediaPlayer.Play;
  HoverMediaPlayer.FileName := AssetPath + '/sound/sfx/click.wav';
  for var Mesh in Player.MeshCollection do
    begin
      Mesh.MaterialSource := LightMaterialSource1;
    end;
  TextureMaterialSource2.Texture := CreateStarBitmap(1000, 1280, 720);
  //Player.RotationCenter.X := 0.35;
  Plane1.Position.X := Camera.Position.X;
  Plane1.Position.Y := Camera.Position.Y;
end;

procedure TTuringWindow.Plane1MouseDown(Sender: TObject; Button: TMouseButton;
  Shift: TShiftState; X, Y: Single; RayPos, RayDir: TVector3D);
begin
  var xd : Double := (Viewport.Width / 2) - X;
  var yd : Double := (Viewport.Height / 2) - Y;
  var result : Double := ArcTan2(yd, xd);
  Player.RotationAngle.Y := RadToDeg(result);
end;

procedure TTuringWindow.Plane1MouseUp(Sender: TObject; Button: TMouseButton;
  Shift: TShiftState; X, Y: Single; RayPos, RayDir: TVector3D);
begin
  buttondown := false;
end;

procedure TTuringWindow.PlayButtonClick(Sender: TObject);
begin
  TabControl.Next(TTabTransition.None);
  MainViewport.Visible := false;
end;

procedure TTuringWindow.PlayButtonMouseEnter(Sender: TObject);
begin
  ButtonHover(Sender as TRectangle);
end;

procedure TTuringWindow.PlayButtonMouseLeave(Sender: TObject);
begin
  ButtonUnhover(Sender as TRectangle);
end;

procedure TTuringWindow.Rectangle4MouseEnter(Sender: TObject);
begin
  ButtonHover(Sender as TRectangle);
end;

procedure TTuringWindow.Rectangle4MouseLeave(Sender: TObject);
begin
  ButtonUnhover(Sender as TRectangle);
end;

procedure TTuringWindow.Sphere1Click(Sender: TObject);
begin
  Layer3D1.Visible := not Layer3d1.Visible;
end;

procedure TTuringWindow.Rectangle15Click(Sender: TObject);
begin
  Sphere2.Visible := true;
  FloatAnimation3.Start;
  FloatAnimation4.Start;
  FloatAnimation5.STarT;
end;

procedure TTuringWindow.Rectangle4Click(Sender: TObject);
begin
  TabControl.GotoVisibleTab(2, TTabTransition.None);
end;

end.

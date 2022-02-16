unit Turing.Classes;

interface

uses
  Turing.Utils,
  System.Generics.Collections,
  FMX.Viewport3D, FMX.Controls3D, FMX.Layers3D, FMX.Objects, FMX.Types,
  FMX.Objects3D, FMX.Types3D, System.JSON, FMX.Dialogs;

  type
    TTuring = class abstract(TObject)
        ID : String;
        Name : String;
    end;

    TTuringStar = class(TTuring)
        Sphere : TSphere;
        //constructor Create(Parent : TViewport3D; const JSON : String);
    end;

    TTuringPlanet = class(TTuring)
        Sphere : TSphere;
        //constructor Create(Parent : TViewport3D; const JSON : String);
    end;

    TTuringPlayer = class(TTuring)
        Model : TModel3D;
        constructor Create(Parent : TFMXObject; const JSON : String);
    end;

    TTuringSystem = class(TTuring)
        Viewport : TViewport3D;
        Camera : TCamera;
        Layer : TLayer3D;
        Background : TImage;
        Planets : TList<TTuringPlanet>;
        constructor Create(Parent : TFMXObject; const JSON: String);
    end;

    TTuringManager = class(TObject)
        Username : String;
        Password : String;
        IP : String;
        System : TTuringSystem;
        constructor Create(const Username, Password, IP, _AssetPath : String; Parent : TFMXObject);
    end;

var
  AssetPath : String;

implementation

constructor TTuringManager.Create(const Username, Password, IP, _AssetPath: String; Parent : TFMXObject);
begin
  AssetPath := _AssetPath;
  System := TTuringSystem.Create(Parent,
  '{' +
    '"id": "s_abcd1234",' +
    '"name" : "Solar System",' +
    '"star" : {' +
        '"name" : "Sun",' +
        '"id": "s_abcd1234",' +
        '"radius" : 1,' +
        '"texture" : "bitmap/textures/sun.jpg",' +
        '"heat" : 6000,' +
        '"rotation" : 10' +
    '},' +
    '"planets" : [' +
        '{' +
            '"name" : "Earth",' +
            '"id": "p_abcd1234",' +
            '"radius": 5,' +
            '"texture": "bitmap/textures/earth_daymap.jpg",' +
            '"x" : 0,' +
            '"y" : 0,' +
            '"rotation" : 24' +
        '}' +
    '],' +
    '"players" : [' +
        '"p_abcd1234"' +
    ']' +
  '}'
  );
end;

constructor TTuringSystem.Create(Parent: TFmxObject; const JSON: string);
begin

  inherited Create();

  Planets := TList<TTuringPlanet>.Create();

  // Create Viewport
  Viewport := TViewport3D.Create(Parent);
  Viewport.Parent := Parent;
  Viewport.Align := TAlignLayout.Client;

  // Create Camera
  Camera := TCamera.Create(Viewport);
  Camera.Parent := Viewport;
  Camera.Position.Z := -10;
  // Update Viewport Camera
  Viewport.Camera := Camera;
  Viewport.UsingDesignCamera := false;

  // Create 2D Layer
  Layer := TLayer3D.Create(Viewport);
  Layer.Parent := Viewport;
  Layer.Projection := TProjection.Screen;
  Layer.Align := TAlignLayout.Client;
  Layer.ZWrite := false;

  // Create background stars
  Background := TImage.Create(Layer);
  Background.Parent := Layer;
  Background.Align := TAlignLayout.Client;
  Background.Bitmap := CreateStarBitmap(1000, Round(Layer.Width), Round(layer.Height));

  var JSONValue : TJSONValue := TJSONObject.ParseJSONValue(JSON);

    (*

    var Star : TJSONObject := JSONValue.GetValue<TJSONObject>('star');

    FStar := TStar.Create(
      FViewport,
      Star.GetValue<String>('id'),
      Star.GetValue<String>('name'),
      Star.GetValue<Integer>('radius'),
      Star.GetValue<Integer>('rotation'),
      Star.GetValue<Integer>('heat')
    );
    *)


    var Planets : TJSONArray := JSONValue.GetValue<TJSONArray>('planets');

    for var Planet : TJSONValue in Planets do
    begin
      //var NewPlanet : TTuringPlanet := TTuringPlanet.Create(Viewport, Planet.ToString);
      //self.Planets.Add(NewPlanet);
    end;

    JSONValue.Free;
end;

constructor TTuringPlayer.Create(Parent: TFmxObject; const JSON: string);
begin
  Model := TModel3D.Create(Parent);
  Model.Parent := Parent;
  Model.LoadFromFile(AssetPath + '\3d\ship_0.obj');
end;

end.
use js_ffi::*;

pub struct BabylonApi {

}

impl BabylonApi {
    pub fn create_basic_scene(selector:&str) -> JSObject {
        let func = register_function(
            r#"
            function(selector){
                var canvas = document.querySelector(selector);
                var engine = new BABYLON.Engine(canvas, true); 
                var createScene = function () {
                    var scene = new BABYLON.Scene(engine);

                    // Add a camera to the scene and attach it to the canvas
                    var camera = new BABYLON.ArcRotateCamera(
                        "Camera",
                        Math.PI / 2,
                        Math.PI / 2,
                        2,
                        BABYLON.Vector3.Zero(),
                        scene
                    );
                    camera.attachControl(canvas, true);

                    // Add lights to the scene
                    var light1 = new BABYLON.HemisphericLight(
                        "light1",
                        new BABYLON.Vector3(1, 1, 0),
                        scene
                    );
                    var light2 = new BABYLON.PointLight(
                        "light2",
                        new BABYLON.Vector3(0, 1, -1),
                        scene
                    );

                    return scene;
                };
                var scene = createScene();
                engine.runRenderLoop(function () {
                        scene.render();
                });
                window.addEventListener("resize", function () {
                        engine.resize();
                });
                return scene;
            }
        "#,
        );
        func.invoke_1(selector).as_owned()
    }

    pub fn create_sphere(scene_ref:&JSObject,size:f32) -> JSObject {
        let func = register_function(
            r#"
            function(scene,size){
                debugger;
                return BABYLON.MeshBuilder.CreateSphere(
                    "sphere",
                    { diameter: size },
                    scene);
            }
        "#,
        );
        func.invoke_2(scene_ref,size).as_owned()
    }
}
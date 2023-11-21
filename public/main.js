function init() {
  let rustApp = null;
  try {
    import("../pkg").then((module) => {
      rustApp = module;
      console.log(rustApp);
    });
  } catch (err) {
    console.error(`Unexpected error: ${err}`);
    return;
  }

  console.log(rustApp);

  const input = document.getElementById("upload");
  const fileReader = new FileReader();

  fileReader.onloadend = () => {
    const base64String = fileReader.result.replace(
      /^data:image\/(png|jpeg|jpg);base64,/,
      ""
    );
    const imageDataUrl = rustApp.grayscale(base64String);
    console.log(imageDataUrl);
    document.getElementById("new-img").setAttribute("src", imageDataUrl);
  };

  input.addEventListener("change", () => {
    const file = input.files[0];
    fileReader.readAsDataURL(file);
  });
}

init();

window.BittyClass = class {
  bittyInit() {
    this.api.fn.setProp("--load-hider", "1");
    this.api.forward(null, "svg");
    this.api.forward(null, "svg2");
  }

  async svg(_event, el) {
    const newSvg = await this.api.getSVG(
      "/svgs/output/for-example-this-one.svg",
    );
    el.appendChild(newSvg.ok);
  }

  async svg2(_event, el) {
    const newSvg = await this.api.getSVG(
      "/svgs/filter-test/1.svg",
    );
    el.appendChild(newSvg.ok);
  }
};

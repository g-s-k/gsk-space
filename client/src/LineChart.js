import React, { Component, Fragment } from "react";

export default class LineChart extends Component {
  constructor(props) {
    super(props);

    this.ref = null;
  }

  setRef = e => (this.ref = e);

  getHeight = () => {
    if (this.ref === null) return 0;
    return this.ref.height.baseVal.value;
  };

  getWidth = () => {
    if (this.ref === null) return 0;
    return this.ref.width.baseVal.value;
  };

  getMinY() {
    const { data } = this.props;
    const minVals = data.map(pt => Math.min(pt.one, pt.five, pt.fifteen));
    return Math.min(...minVals);
  }

  getMaxY() {
    const { data } = this.props;
    const maxVals = data.map(pt => Math.max(pt.one, pt.five, pt.fifteen));
    return Math.max(...maxVals);
  }

  getSvgX(x) {
    return this.getWidth() - x * 2;
  }

  getSvgY(y) {
    const height = this.getHeight();
    return height - (y / this.getMaxY()) * height;
  }

  makePath(d, color) {
    const pathD = `M ${d
      .map((pt, idx) => `${this.getSvgX(idx)} ${this.getSvgY(pt)}`)
      .join(" L ")}`;
    return (
      <path className="LineChart_Path" d={pathD} style={{ stroke: color }} />
    );
  }

  makeAxis() {
    const height = this.getHeight();
    const width = this.getWidth();

    const xVals = Array(Math.floor(width / 60))
      .fill(0)
      .map((v, i) => {
        const xval = this.getSvgX(i * 60);
        return (
          <Fragment key={i}>
            <text x={xval + 5} y={height - 5}>
              {i}
            </text>
            <line
              x1={xval}
              x2={xval}
              y1={0}
              y2={height}
              stroke-dasharray="10,10"
            />
          </Fragment>
        );
      });

    const yVals = Array(Math.max(0, Math.floor(this.getMaxY() * 2)))
      .fill(0)
      .map((v, i) => {
        if (i === 0) return null;
        const yval = this.getSvgY(i / 2);
        return (
          <Fragment key={i}>
            <text x={width - 30} y={yval - 5}>
              {i / 2}
            </text>
            <line
              x1={0}
              x2={width}
              y1={yval}
              y2={yval}
              stroke-dasharray="10,10"
            />
          </Fragment>
        );
      });

    return (
      <g className="LineChart_Axis">
        {xVals}
        {yVals}
      </g>
    );
  }

  render() {
    const { className, data } = this.props;
    if (!data) return null;
    return (
      <svg className={`${className} LineChart`} ref={this.setRef}>
        {this.makePath(data.map(x => x.fifteen), "blue")}
        {this.makePath(data.map(x => x.five), "green")}
        {this.makePath(data.map(x => x.one), "red")}
        {this.makeAxis()}
      </svg>
    );
  }
}

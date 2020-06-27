import React, { Component } from 'react';

// See https://create-react-app.dev/docs/adding-images-fonts-and-files/
// import logo from './symbols-1.svg';
import { ReactComponent as Logo } from './lemmy.svg';

export class Symbols extends Component<any, any> {
  constructor(props: any, context: any) {
    super(props, context);
  }

  render() {
    return (
      // <img src={logo} alt="logo" />
      <Logo />
    );
  }
}

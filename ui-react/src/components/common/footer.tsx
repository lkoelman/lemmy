import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import { repoUrl } from '../../utils';
import { i18n } from '../../i18next';

const { version } = require('../../../package.json');

export class Footer extends Component<any, any> {
  constructor(props: any, context: any) {
    super(props, context);
  }

  render() {
    return (
      <nav className="container navbar navbar-expand-md navbar-light navbar-bg p-0 px-3 mt-2">
        <div className="navbar-collapse">
          <ul className="navbar-nav ml-auto">
            <li className="nav-item">
              <span className="navbar-text">{version}</span>
            </li>
            <li className="nav-item">
              <Link className="nav-link" to="/modlog">
                {i18n.t('modlog')}
              </Link>
            </li>
            <li className="nav-item">
              <a className="nav-link" href={'/docs/index.html'}>
                {i18n.t('docs')}
              </a>
            </li>
            <li className="nav-item">
              <Link className="nav-link" to="/sponsors">
                {i18n.t('donate')}
              </Link>
            </li>
            <li className="nav-item">
              <a className="nav-link" href={repoUrl}>
                {i18n.t('code')}
              </a>
            </li>
          </ul>
        </div>
      </nav>
    );
  }
}

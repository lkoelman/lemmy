import React, { Component } from 'react';
import { Subscription } from 'rxjs';
import { retryWhen, delay, take } from 'rxjs/operators';
import {
  RegisterForm,
  LoginResponse,
  UserOperation,
  WebSocketJsonResponse,
} from '../../interfaces';
import { WebSocketService, UserService } from '../../services';
import { wsJsonToRes, toast } from '../../utils';
import { SiteForm } from '../communities/site-form';
import { i18n } from '../../i18next';

interface State {
  userForm: RegisterForm;
  doneRegisteringUser: boolean;
  userLoading: boolean;
}

export class Setup extends Component<any, State> {
  private subscription: Subscription;

  private emptyState: State = {
    userForm: {
      username: undefined,
      password: undefined,
      password_verify: undefined,
      admin: true,
      show_nsfw: true,
    },
    doneRegisteringUser: false,
    userLoading: false,
  };

  constructor(props: any, context: any) {
    super(props, context);

    this.state = this.emptyState;

    this.subscription = WebSocketService.Instance.subject
      .pipe(retryWhen(errors => errors.pipe(delay(3000), take(10))))
      .subscribe(
        msg => this.parseMessage(msg),
        err => console.error(err),
        () => console.log('complete')
      );
  }

  componentWillUnmount() {
    this.subscription.unsubscribe();
  }

  componentDidMount() {
    document.title = `${i18n.t('setup')} - Lemmy`;
  }

  render() {
    return (
      <div className="container">
        <div className="row">
          <div className="col-12 offset-lg-3 col-lg-6">
            <h3>{i18n.t('lemmy_instance_setup')}</h3>
            {!this.state.doneRegisteringUser ? (
              this.registerUser()
            ) : (
              <SiteForm />
            )}
          </div>
        </div>
      </div>
    );
  }

  registerUser() {
    return (
      <form onSubmit={this.handleRegisterSubmit}>
        <h5>{i18n.t('setup_admin')}</h5>
        <div className="form-group row">
          <label className="col-sm-2 col-form-label" htmlFor="username">
            {i18n.t('username')}
          </label>
          <div className="col-sm-10">
            <input
              type="text"
              className="form-control"
              id="username"
              value={this.state.userForm.username}
              onInput={this.handleRegisterUsernameChange}
              required
              minLength={3}
              maxLength={20}
              pattern="[a-zA-Z0-9_]+"
            />
          </div>
        </div>
        <div className="form-group row">
          <label className="col-sm-2 col-form-label" htmlFor="email">
            {i18n.t('email')}
          </label>

          <div className="col-sm-10">
            <input
              type="email"
              id="email"
              className="form-control"
              placeholder={i18n.t('optional')}
              value={this.state.userForm.email}
              onInput={this.handleRegisterEmailChange}
              minLength={3}
            />
          </div>
        </div>
        <div className="form-group row">
          <label className="col-sm-2 col-form-label" htmlFor="password">
            {i18n.t('password')}
          </label>
          <div className="col-sm-10">
            <input
              type="password"
              id="password"
              value={this.state.userForm.password}
              onInput={this.handleRegisterPasswordChange}
              className="form-control"
              required
            />
          </div>
        </div>
        <div className="form-group row">
          <label className="col-sm-2 col-form-label" htmlFor="verify-password">
            {i18n.t('verify_password')}
          </label>
          <div className="col-sm-10">
            <input
              type="password"
              id="verify-password"
              value={this.state.userForm.password_verify}
              onInput={this.handleRegisterPasswordVerifyChange}
              className="form-control"
              required
            />
          </div>
        </div>
        <div className="form-group row">
          <div className="col-sm-10">
            <button type="submit" className="btn btn-secondary">
              {this.state.userLoading ? (
                <svg className="icon icon-spinner spin">
                  <use xlinkHref="#icon-spinner"></use>
                </svg>
              ) : (
                i18n.t('sign_up')
              )}
            </button>
          </div>
        </div>
      </form>
    );
  }

  handleRegisterSubmit = (event: any) => {
    event.preventDefault();
    this.setState({ userLoading: true});
    event.preventDefault();
    WebSocketService.Instance.register(this.state.userForm);
  }

  handleRegisterUsernameChange = (event: any) => {
    this.state.userForm.username = event.target.value;
    this.setState(this.state);
  }

  handleRegisterEmailChange = (event: any) => {
    this.state.userForm.email = event.target.value;
    this.setState(this.state);
  }

  handleRegisterPasswordChange = (event: any) => {
    this.state.userForm.password = event.target.value;
    this.setState(this.state);
  }

  handleRegisterPasswordVerifyChange = (event: any) => {
    this.state.userForm.password_verify = event.target.value;
    this.setState(this.state);
  }

  parseMessage(msg: WebSocketJsonResponse) {
    let res = wsJsonToRes(msg);
    if (msg.error) {
      toast(i18n.t(msg.error), 'danger');
      this.setState({ userLoading: false });
      return;
    } else if (res.op == UserOperation.Register) {
      let data = res.data as LoginResponse;
      UserService.Instance.login(data);
      this.setState({
        userLoading: false,
        doneRegisteringUser: true,
      });
    } else if (res.op == UserOperation.CreateSite) {
      this.props.history.push('/');
    }
  }
}

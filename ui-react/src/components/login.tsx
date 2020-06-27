import React, { Component } from 'react';
import { Subscription } from 'rxjs';
import { retryWhen, delay, take } from 'rxjs/operators';
import {
  LoginForm,
  RegisterForm,
  LoginResponse,
  UserOperation,
  PasswordResetForm,
  GetSiteResponse,
  WebSocketJsonResponse,
} from '../interfaces';
import { WebSocketService, UserService } from '../services';
import { wsJsonToRes, validEmail, toast } from '../utils';
import { i18n } from '../i18next';

interface State {
  loginForm: LoginForm;
  registerForm: RegisterForm;
  loginLoading: boolean;
  registerLoading: boolean;
  enable_nsfw: boolean;
}

export class Login extends Component<any, State> {
  private subscription: Subscription;

  emptyState: State = {
    loginForm: {
      username_or_email: undefined,
      password: undefined,
    },
    registerForm: {
      username: undefined,
      password: undefined,
      password_verify: undefined,
      admin: false,
      show_nsfw: false,
    },
    loginLoading: false,
    registerLoading: false,
    enable_nsfw: undefined,
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

    WebSocketService.Instance.getSite();
  }

  componentWillUnmount() {
    this.subscription.unsubscribe();
  }

  render() {
    return (
      <div className="container">
        <div className="row">
          <div className="col-12 col-lg-6 mb-4">{this.loginForm()}</div>
          <div className="col-12 col-lg-6">{this.registerForm()}</div>
        </div>
      </div>
    );
  }

  loginForm() {
    return (
      <div>
        <form onSubmit={this.handleLoginSubmit}>
          <h5>{i18n.t('login')}</h5>
          <div className="form-group row">
            <label
              className="col-sm-2 col-form-label"
              htmlFor="login-email-or-username"
            >
              {i18n.t('email_or_username')}
            </label>
            <div className="col-sm-10">
              <input
                type="text"
                className="form-control"
                id="login-email-or-username"
                value={this.state.loginForm.username_or_email}
                onInput={this.handleLoginUsernameChange}
                required
                minLength={3}
              />
            </div>
          </div>
          <div className="form-group row">
            <label className="col-sm-2 col-form-label" htmlFor="login-password">
              {i18n.t('password')}
            </label>
            <div className="col-sm-10">
              <input
                type="password"
                id="login-password"
                value={this.state.loginForm.password}
                onInput={this.handleLoginPasswordChange}
                className="form-control"
                required
              />
              <button
                disabled={!validEmail(this.state.loginForm.username_or_email)}
                onClick={this.handlePasswordReset}
                className="btn p-0 btn-link d-inline-block float-right text-muted small font-weight-bold"
              >
                {i18n.t('forgot_password')}
              </button>
            </div>
          </div>
          <div className="form-group row">
            <div className="col-sm-10">
              <button type="submit" className="btn btn-secondary">
                {this.state.loginLoading ? (
                  <svg className="icon icon-spinner spin">
                    <use xlinkHref="#icon-spinner"></use>
                  </svg>
                ) : (
                  i18n.t('login')
                )}
              </button>
            </div>
          </div>
        </form>
      </div>
    );
  }
  registerForm() {
    return (
      <form onSubmit={this.handleRegisterSubmit}>
        <h5>{i18n.t('sign_up')}</h5>

        <div className="form-group row">
          <label className="col-sm-2 col-form-label" htmlFor="register-username">
            {i18n.t('username')}
          </label>

          <div className="col-sm-10">
            <input
              type="text"
              id="register-username"
              className="form-control"
              value={this.state.registerForm.username}
              onInput={this.handleRegisterUsernameChange}
              required
              minLength={3}
              maxLength={20}
              pattern="[a-zA-Z0-9_]+"
            />
          </div>
        </div>

        <div className="form-group row">
          <label className="col-sm-2 col-form-label" htmlFor="register-email">
            {i18n.t('email')}
          </label>
          <div className="col-sm-10">
            <input
              type="email"
              id="register-email"
              className="form-control"
              placeholder={i18n.t('optional')}
              value={this.state.registerForm.email}
              onInput={this.handleRegisterEmailChange}
              minLength={3}
            />
          </div>
        </div>

        <div className="form-group row">
          <label className="col-sm-2 col-form-label" htmlFor="register-password">
            {i18n.t('password')}
          </label>
          <div className="col-sm-10">
            <input
              type="password"
              id="register-password"
              value={this.state.registerForm.password}
              autoComplete="new-password"
              onInput={this.handleRegisterPasswordChange}
              className="form-control"
              required
            />
          </div>
        </div>

        <div className="form-group row">
          <label
            className="col-sm-2 col-form-label"
            htmlFor="register-verify-password"
          >
            {i18n.t('verify_password')}
          </label>
          <div className="col-sm-10">
            <input
              type="password"
              id="register-verify-password"
              value={this.state.registerForm.password_verify}
              autoComplete="new-password"
              onInput={this.handleRegisterPasswordVerifyChange}
              className="form-control"
              required
            />
          </div>
        </div>
        {this.state.enable_nsfw && (
          <div className="form-group row">
            <div className="col-sm-10">
              <div className="form-check">
                <input
                  className="form-check-input"
                  id="register-show-nsfw"
                  type="checkbox"
                  checked={this.state.registerForm.show_nsfw}
                  onChange={this.handleRegisterShowNsfwChange}
                />
                <label className="form-check-label" htmlFor="register-show-nsfw">
                  {i18n.t('show_nsfw')}
                </label>
              </div>
            </div>
          </div>
        )}
        <div className="form-group row">
          <div className="col-sm-10">
            <button type="submit" className="btn btn-secondary">
              {this.state.registerLoading ? (
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

  handleLoginSubmit = (event: any) => {
    event.preventDefault();
    this.setState({ loginLoading : true });
    this.setState(this.state);
    WebSocketService.Instance.login(this.state.loginForm);
  }

  handleLoginUsernameChange = (event: any) => {
    this.state.loginForm.username_or_email = event.target.value;
    this.setState(this.state);
  }

  handleLoginPasswordChange = (event: any) => {
    this.state.loginForm.password = event.target.value;
    this.setState(this.state);
  }

  handleRegisterSubmit = (event: any) => {
    event.preventDefault();
    this.setState({ registerLoading : true });
    this.setState(this.state);

    WebSocketService.Instance.register(this.state.registerForm);
  }

  handleRegisterUsernameChange = (event: any) => {
    this.state.registerForm.username = event.target.value;
    this.setState(this.state);
  }

  handleRegisterEmailChange = (event: any) => {
    this.state.registerForm.email = event.target.value;
    if (this.state.registerForm.email == '') {
      this.state.registerForm.email = undefined;
    }
    this.setState(this.state);
  }

  handleRegisterPasswordChange = (event: any) => {
    this.state.registerForm.password = event.target.value;
    this.setState(this.state);
  }

  handleRegisterPasswordVerifyChange = (event: any) => {
    this.state.registerForm.password_verify = event.target.value;
    this.setState(this.state);
  }

  handleRegisterShowNsfwChange = (event: any) => {
    this.state.registerForm.show_nsfw = event.target.checked;
    this.setState(this.state);
  }

  handlePasswordReset = (event: any) => {
    event.preventDefault();
    let resetForm: PasswordResetForm = {
      email: this.state.loginForm.username_or_email,
    };
    WebSocketService.Instance.passwordReset(resetForm);
  }

  parseMessage(msg: WebSocketJsonResponse) {
    let res = wsJsonToRes(msg);
    if (msg.error) {
      toast(i18n.t(msg.error), 'danger');
      this.setState(this.emptyState);
      return;
    } else {
      if (res.op == UserOperation.Login) {
        let data = res.data as LoginResponse;
        this.setState(this.emptyState);
        UserService.Instance.login(data);
        WebSocketService.Instance.userJoin();
        toast(i18n.t('logged_in'));
        this.props.history.push('/');
      } else if (res.op == UserOperation.Register) {
        let data = res.data as LoginResponse;
        this.setState(this.emptyState);
        UserService.Instance.login(data);
        WebSocketService.Instance.userJoin();
        this.props.history.push('/communities');
      } else if (res.op == UserOperation.PasswordReset) {
        toast(i18n.t('reset_password_mail_sent'));
      } else if (res.op == UserOperation.GetSite) {
        let data = res.data as GetSiteResponse;
        this.setState({ enable_nsfw : data.site.enable_nsfw });
        document.title = `${i18n.t('login')} - ${
          WebSocketService.Instance.site.name
        }`;
      }
    }
  }
}

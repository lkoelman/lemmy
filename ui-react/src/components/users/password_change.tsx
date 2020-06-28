import React, { Component } from 'react';
import { Subscription } from 'rxjs';
import { retryWhen, delay, take } from 'rxjs/operators';
import {
  UserOperation,
  LoginResponse,
  PasswordChangeForm,
  WebSocketJsonResponse,
} from '../../interfaces';
import { WebSocketService, UserService } from '../../services';
import { wsJsonToRes, capitalizeFirstLetter, toast } from '../../utils';
import { i18n } from '../../i18next';

interface State {
  passwordChangeForm: PasswordChangeForm;
  loading: boolean;
}

export class PasswordChange extends Component<any, State> {
  private subscription: Subscription;

  emptyState: State = {
    passwordChangeForm: {
      token: this.props.match.params.token,
      password: undefined,
      password_verify: undefined,
    },
    loading: false,
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
    document.title = `${i18n.t('password_change')} - ${
      WebSocketService.Instance.site.name
    }`;
  }

  render() {
    return (
      <div className="container">
        <div className="row">
          <div className="col-12 col-lg-6 offset-lg-3 mb-4">
            <h5>{i18n.t('password_change')}</h5>
            {this.passwordChangeForm()}
          </div>
        </div>
      </div>
    );
  }

  passwordChangeForm() {
    return (
      <form onSubmit={this.handlePasswordChangeSubmit}>
        <div className="form-group row">
          <label className="col-sm-2 col-form-label">
            {i18n.t('new_password')}
          </label>
          <div className="col-sm-10">
            <input
              type="password"
              value={this.state.passwordChangeForm.password}
              onInput={this.handlePasswordChange}
              className="form-control"
              required
            />
          </div>
        </div>
        <div className="form-group row">
          <label className="col-sm-2 col-form-label">
            {i18n.t('verify_password')}
          </label>
          <div className="col-sm-10">
            <input
              type="password"
              value={this.state.passwordChangeForm.password_verify}
              onInput={this.handleVerifyPasswordChange}
              className="form-control"
              required
            />
          </div>
        </div>
        <div className="form-group row">
          <div className="col-sm-10">
            <button type="submit" className="btn btn-secondary">
              {this.state.loading ? (
                <svg className="icon icon-spinner spin">
                  <use xlinkHref="#icon-spinner"></use>
                </svg>
              ) : (
                capitalizeFirstLetter(i18n.t('save'))
              )}
            </button>
          </div>
        </div>
      </form>
    );
  }

  handlePasswordChange = (event: any) => {
    this.state.passwordChangeForm.password = event.target.value;
    this.setState(this.state);
  }

  handleVerifyPasswordChange = (event: any) => {
    this.state.passwordChangeForm.password_verify = event.target.value;
    this.setState(this.state);
  }

  handlePasswordChangeSubmit = (event: any) => {
    event.preventDefault();
    this.setState({ loading: true });

    WebSocketService.Instance.passwordChange(this.state.passwordChangeForm);
  }

  parseMessage(msg: WebSocketJsonResponse) {
    let res = wsJsonToRes(msg);
    if (msg.error) {
      toast(i18n.t(msg.error), 'danger');
      this.setState({ loading: false });
      return;
    } else {
      if (res.op == UserOperation.PasswordChange) {
        let data = res.data as LoginResponse;
        this.state = this.emptyState;
        this.setState(this.state);
        UserService.Instance.login(data);
        this.props.history.push('/');
      }
    }
  }
}

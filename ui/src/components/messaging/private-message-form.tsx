import React, { Component } from 'react';
import { Prompt, Link } from 'react-router-dom';

import { Subscription } from 'rxjs';
import { retryWhen, delay, take } from 'rxjs/operators';
import {
  PrivateMessageForm as PrivateMessageFormI,
  EditPrivateMessageForm,
  PrivateMessageFormParams,
  PrivateMessage,
  PrivateMessageResponse,
  UserView,
  UserOperation,
  UserDetailsResponse,
  GetUserDetailsForm,
  SortType,
  WebSocketJsonResponse,
} from '../../interfaces';
import { WebSocketService } from '../../services';
import {
  capitalizeFirstLetter,
  markdownHelpUrl,
  mdToHtml,
  wsJsonToRes,
  toast,
  randomStr,
  setupTribute,
  setupTippy,
} from '../../utils';
import { UserListing } from '../users/user-listing';
import Tribute from 'tributejs/src/Tribute.js';
import autosize from 'autosize';
import { i18n } from '../../i18next';
import { Trans } from 'react-i18next';

interface PrivateMessageFormProps {
  privateMessage?: PrivateMessage; // If a pm is given, that means this is an edit
  params?: PrivateMessageFormParams;
  onCancel?(): any;
  onCreate?(message: PrivateMessage): any;
  onEdit?(message: PrivateMessage): any;
}

interface PrivateMessageFormState {
  privateMessageForm: PrivateMessageFormI;
  recipient: UserView;
  loading: boolean;
  previewMode: boolean;
  showDisclaimer: boolean;
}

export class PrivateMessageForm extends Component<
  PrivateMessageFormProps,
  PrivateMessageFormState
> {
  private id = `message-form-${randomStr()}`;
  private tribute: Tribute;
  private subscription: Subscription;
  private emptyState: PrivateMessageFormState = {
    privateMessageForm: {
      content: null,
      recipient_id: null,
    },
    recipient: null,
    loading: false,
    previewMode: false,
    showDisclaimer: false,
  };

  constructor(props: any, context: any) {
    super(props, context);

    this.tribute = setupTribute();
    this.state = this.emptyState;

    if (this.props.privateMessage) {
      this.setState({ privateMessageForm: {
        content: this.props.privateMessage.content,
        recipient_id: this.props.privateMessage.recipient_id,
      }});
    }

    if (this.props.params) {
      this.setState({ privateMessageForm: {
        ...this.state.privateMessageForm,
        recipient_id: this.props.params.recipient_id,
      }});
      let form: GetUserDetailsForm = {
        user_id: this.state.privateMessageForm.recipient_id,
        sort: SortType[SortType.New],
        saved_only: false,
      };
      WebSocketService.Instance.getUserDetails(form);
    }

    this.subscription = WebSocketService.Instance.subject
      .pipe(retryWhen(errors => errors.pipe(delay(3000), take(10))))
      .subscribe(
        msg => this.parseMessage(msg),
        err => console.error(err),
        () => console.log('complete')
      );
  }

  componentDidMount() {
    var textarea: any = document.getElementById(this.id);
    autosize(textarea);
    this.tribute.attach(textarea);
    textarea.addEventListener('tribute-replaced', () => {
      this.state.privateMessageForm.content = textarea.value;
      this.setState(this.state);
      autosize.update(textarea);
    });
    setupTippy();
  }

  componentWillUnmount() {
    this.subscription.unsubscribe();
  }

  render() {
    return (
      <div>
        <Prompt
          when={!this.state.loading && Boolean(this.state.privateMessageForm.content)}
          message={i18n.t('block_leaving')}
        />
        <form onSubmit={this.handlePrivateMessageSubmit}>
          {!this.props.privateMessage && (
            <div className="form-group row">
              <label className="col-sm-2 col-form-label">
                {capitalizeFirstLetter(i18n.t('to'))}
              </label>

              {this.state.recipient && (
                <div className="col-sm-10 form-control-plaintext">
                  <UserListing
                    user={{
                      name: this.state.recipient.name,
                      avatar: this.state.recipient.avatar,
                    }}
                  />
                </div>
              )}
            </div>
          )}
          <div className="form-group row">
            <label className="col-sm-2 col-form-label">{i18n.t('message')}</label>
            <div className="col-sm-10">
              <textarea
                id={this.id}
                value={this.state.privateMessageForm.content}
                onInput={this.handleContentChange}
                className={`form-control ${this.state.previewMode && 'd-none'}`}
                rows={4}
                maxLength={10000}
              />
              {this.state.previewMode && (
                <div
                  className="md-div"
                  dangerouslySetInnerHTML={mdToHtml(
                    this.state.privateMessageForm.content
                  )}
                />
              )}
            </div>
          </div>

          {this.state.showDisclaimer && (
            <div className="form-group row">
              <div className="offset-sm-2 col-sm-10">
                <div className="alert alert-danger" role="alert">
                  <Trans i18nKey="private_message_disclaimer">
                    #
                    <a
                      className="alert-link"
                      target="_blank"
                      href="https://about.riot.im/"
                    >
                      #
                    </a>
                  </Trans>
                </div>
              </div>
            </div>
          )}
          <div className="form-group row">
            <div className="offset-sm-2 col-sm-10">
              <button type="submit" className="btn btn-secondary mr-2">
                {this.state.loading ? (
                  <svg className="icon icon-spinner spin">
                    <use xlinkHref="#icon-spinner"></use>
                  </svg>
                ) : this.props.privateMessage ? (
                  capitalizeFirstLetter(i18n.t('save'))
                ) : (
                  capitalizeFirstLetter(i18n.t('send_message'))
                )}
              </button>
              {this.state.privateMessageForm.content && (
                <button
                  className={`btn btn-secondary mr-2 ${
                    this.state.previewMode && 'active'
                  }`}
                  onClick={this.handlePreviewToggle}
                >
                  {i18n.t('preview')}
                </button>
              )}
              {this.props.privateMessage && (
                <button
                  type="button"
                  className="btn btn-secondary"
                  onClick={this.handleCancel}
                >
                  {i18n.t('cancel')}
                </button>
              )}
              <ul className="d-inline-block float-right list-inline mb-1 text-muted font-weight-bold">
                <li className="list-inline-item">
                  <span
                    onClick={this.handleShowDisclaimer}
                    className="pointer"
                    data-tippy-content={i18n.t('disclaimer')}
                  >
                    <svg className={`icon icon-inline`}>
                      <use xlinkHref="#icon-alert-triangle"></use>
                    </svg>
                  </span>
                </li>
                <li className="list-inline-item">
                  <a
                    href={markdownHelpUrl}
                    target="_blank"
                    className="text-muted"
                    title={i18n.t('formatting_help')}
                  >
                    <svg className="icon icon-inline">
                      <use xlinkHref="#icon-help-circle"></use>
                    </svg>
                  </a>
                </li>
              </ul>
            </div>
          </div>
        </form>
      </div>
    );
  }

  handlePrivateMessageSubmit = (event: any) => {
    event.preventDefault();
    if (this.props.privateMessage) {
      let editForm: EditPrivateMessageForm = {
        edit_id: this.props.privateMessage.id,
        content: this.state.privateMessageForm.content,
      };
      WebSocketService.Instance.editPrivateMessage(editForm);
    } else {
      WebSocketService.Instance.createPrivateMessage(
        this.state.privateMessageForm
      );
    }
    this.setState({ loading: false});
  }

  handleRecipientChange = (event: any) => {
    this.setState({ recipient: event.target.value });
  }

  handleContentChange = (event: any) => {
    this.state.privateMessageForm.content = event.target.value;
    this.setState(this.state);
  }

  handleCancel = () => {
    this.props.onCancel();
  }

  handlePreviewToggle = (event: any) => {
    event.preventDefault();
    this.setState({ previewMode: !this.state.previewMode });
  }

  handleShowDisclaimer = () => {
    this.setState({ showDisclaimer: !this.state.showDisclaimer });
  }

  parseMessage(msg: WebSocketJsonResponse) {
    let res = wsJsonToRes(msg);
    if (msg.error) {
      toast(i18n.t(msg.error), 'danger');
      this.setState({ loading: false});
      return;
    } else if (res.op == UserOperation.EditPrivateMessage) {
      let data = res.data as PrivateMessageResponse;
      this.setState({ loading: false});
      this.props.onEdit(data.message);
    } else if (res.op == UserOperation.GetUserDetails) {
      let data = res.data as UserDetailsResponse;
      this.state.privateMessageForm.recipient_id = data.user.id;
      this.setState({ recipient: data.user });
    } else if (res.op == UserOperation.CreatePrivateMessage) {
      let data = res.data as PrivateMessageResponse;
      this.setState({ loading: false});
      this.props.onCreate(data.message);
    }
  }
}

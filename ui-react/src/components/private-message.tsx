import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import {
  PrivateMessage as PrivateMessageI,
  EditPrivateMessageForm,
} from '../interfaces';
import { WebSocketService, UserService } from '../services';
import {
  mdToHtml,
  pictshareAvatarThumbnail,
  showAvatars,
  toast,
} from '../utils';
import { MomentTime } from './moment-time';
import { PrivateMessageForm } from './private-message-form';
import { i18n } from '../i18next';

interface PrivateMessageState {
  showReply: boolean;
  showEdit: boolean;
  collapsed: boolean;
  viewSource: boolean;
}

interface PrivateMessageProps {
  privateMessage: PrivateMessageI;
}

export class PrivateMessage extends Component<
  PrivateMessageProps,
  PrivateMessageState
> {
  private emptyState: PrivateMessageState = {
    showReply: false,
    showEdit: false,
    collapsed: false,
    viewSource: false,
  };

  constructor(props: any, context: any) {
    super(props, context);

    this.state = this.emptyState;
  }

  get mine(): boolean {
    return UserService.Instance.user.id == this.props.privateMessage.creator_id;
  }

  render() {
    let message = this.props.privateMessage;
    return (
      <div className="border-top border-light">
        <div>
          <ul className="list-inline mb-0 text-muted small">
            {/* TODO refactor this */}
            <li className="list-inline-item">
              {this.mine ? i18n.t('to') : i18n.t('from')}
            </li>
            <li className="list-inline-item">
              <Link
                className="text-body font-weight-bold"
                to={
                  this.mine
                    ? `/u/${message.recipient_name}`
                    : `/u/${message.creator_name}`
                }
              >
                {(this.mine
                  ? message.recipient_avatar
                  : message.creator_avatar) &&
                  showAvatars() && (
                    <img
                      height="32"
                      width="32"
                      src={pictshareAvatarThumbnail(
                        this.mine
                          ? message.recipient_avatar
                          : message.creator_avatar
                      )}
                      className="rounded-circle mr-1"
                    />
                  )}
                <span>
                  {this.mine ? message.recipient_name : message.creator_name}
                </span>
              </Link>
            </li>
            <li className="list-inline-item">
              <span>
                <MomentTime data={message} />
              </span>
            </li>
            <li className="list-inline-item">
              <div
                className="pointer text-monospace"
                onClick={this.handleMessageCollapse}
              >
                {this.state.collapsed ? (
                  <svg className="icon icon-inline">
                    <use xlinkHref="#icon-plus-square"></use>
                  </svg>
                ) : (
                  <svg className="icon icon-inline">
                    <use xlinkHref="#icon-minus-square"></use>
                  </svg>
                )}
              </div>
            </li>
          </ul>
          {this.state.showEdit && (
            <PrivateMessageForm
              privateMessage={message}
              onEdit={this.handlePrivateMessageEdit}
              onCancel={this.handleReplyCancel}
            />
          )}
          {!this.state.showEdit && !this.state.collapsed && (
            <div>
              {this.state.viewSource ? (
                <pre>{this.messageUnlessRemoved}</pre>
              ) : (
                <div
                  className="md-div"
                  dangerouslySetInnerHTML={mdToHtml(this.messageUnlessRemoved)}
                />
              )}
              <ul className="list-inline mb-0 text-muted font-weight-bold">
                {!this.mine && (
                  <>
                    <li className="list-inline-item">
                      <button
                        className="btn btn-link btn-sm btn-animate text-muted"
                        onClick={this.handleMarkRead}
                        data-tippy-content={
                          message.read
                            ? i18n.t('mark_as_unread')
                            : i18n.t('mark_as_read')
                        }
                      >
                        <svg
                          className={`icon icon-inline ${message.read &&
                            'text-success'}`}
                        >
                          <use xlinkHref="#icon-check"></use>
                        </svg>
                      </button>
                    </li>
                    <li className="list-inline-item">
                      <button
                        className="btn btn-link btn-sm btn-animate text-muted"
                        onClick={this.handleReplyClick}
                        data-tippy-content={i18n.t('reply')}
                      >
                        <svg className="icon icon-inline">
                          <use xlinkHref="#icon-reply1"></use>
                        </svg>
                      </button>
                    </li>
                  </>
                )}
                {this.mine && (
                  <>
                    <li className="list-inline-item">
                      <button
                        className="btn btn-link btn-sm btn-animate text-muted"
                        onClick={this.handleEditClick}
                        data-tippy-content={i18n.t('edit')}
                      >
                        <svg className="icon icon-inline">
                          <use xlinkHref="#icon-edit"></use>
                        </svg>
                      </button>
                    </li>
                    <li className="list-inline-item">
                      <button
                        className="btn btn-link btn-sm btn-animate text-muted"
                        onClick={this.handleDeleteClick}
                        data-tippy-content={
                          !message.deleted
                            ? i18n.t('delete')
                            : i18n.t('restore')
                        }
                      >
                        <svg
                          className={`icon icon-inline ${message.deleted &&
                            'text-danger'}`}
                        >
                          <use xlinkHref="#icon-trash"></use>
                        </svg>
                      </button>
                    </li>
                  </>
                )}
                <li className="list-inline-item">
                  <button
                    className="btn btn-link btn-sm btn-animate text-muted"
                    onClick={this.handleViewSource}
                    data-tippy-content={i18n.t('view_source')}
                  >
                    <svg
                      className={`icon icon-inline ${this.state.viewSource &&
                        'text-success'}`}
                    >
                      <use xlinkHref="#icon-file-text"></use>
                    </svg>
                  </button>
                </li>
              </ul>
            </div>
          )}
        </div>
        {this.state.showReply && (
          <PrivateMessageForm
            params={{
              recipient_id: this.props.privateMessage.creator_id,
            }}
            onCreate={this.handlePrivateMessageCreate}
          />
        )}
        {/* A collapsed clearfix */}
        {this.state.collapsed && <div className="row col-12"></div>}
      </div>
    );
  }

  get messageUnlessRemoved(): string {
    let message = this.props.privateMessage;
    return message.deleted ? `*${i18n.t('deleted')}*` : message.content;
  }

  handleReplyClick = () => {
    this.setState({ showReply : true });
  }

  handleEditClick = () => {
    this.setState({ showEdit : true });
  }

  handleDeleteClick = () => {
    let form: EditPrivateMessageForm = {
      edit_id: this.props.privateMessage.id,
      deleted: !this.props.privateMessage.deleted,
    };
    WebSocketService.Instance.editPrivateMessage(form);
  }

  handleReplyCancel = () => {
    this.setState({ showReply : false });
    this.setState({ showEdit : false });
  }

  handleMarkRead = () => {
    let form: EditPrivateMessageForm = {
      edit_id: this.props.privateMessage.id,
      read: !this.props.privateMessage.read,
    };
    WebSocketService.Instance.editPrivateMessage(form);
  }

  handleMessageCollapse = () => {
    this.setState({ collapsed: !this.state.collapsed});
  }

  handleViewSource = () => {
    this.setState({ viewSource: !this.state.viewSource});
  }

  handlePrivateMessageEdit = () => {
    this.setState({ showEdit : false });
  }

  handlePrivateMessageCreate = () => {
    this.setState({ showReply : false });
    toast(i18n.t('message_sent'));
  }
}

import React, { Component } from 'react';
import { Subscription } from 'rxjs';
import { retryWhen, delay, take } from 'rxjs/operators';
import {
  UserOperation,
  Comment,
  SortType,
  GetRepliesForm,
  GetRepliesResponse,
  GetUserMentionsForm,
  GetUserMentionsResponse,
  UserMentionResponse,
  CommentResponse,
  WebSocketJsonResponse,
  PrivateMessage as PrivateMessageI,
  GetPrivateMessagesForm,
  PrivateMessagesResponse,
  PrivateMessageResponse,
} from '../../interfaces';
import { WebSocketService, UserService } from '../../services';
import {
  wsJsonToRes,
  fetchLimit,
  isCommentType,
  toast,
  editCommentRes,
  saveCommentRes,
  createCommentLikeRes,
  commentsToFlatNodes,
  setupTippy,
} from '../../utils';
import { CommentNodes } from '../comments/comment-nodes';
import { PrivateMessage } from './private-message';
import { SortSelect } from '../listings-page/sort-select';
import { i18n } from '../../i18next';

enum UnreadOrAll {
  Unread,
  All,
}

enum MessageType {
  All,
  Replies,
  Mentions,
  Messages,
}

type ReplyType = Comment | PrivateMessageI;

interface InboxState {
  unreadOrAll: UnreadOrAll;
  messageType: MessageType;
  replies: Array<Comment>;
  mentions: Array<Comment>;
  messages: Array<PrivateMessageI>;
  sort: SortType;
  page: number;
}

export class Inbox extends Component<any, InboxState> {
  private subscription: Subscription;
  private emptyState: InboxState = {
    unreadOrAll: UnreadOrAll.Unread,
    messageType: MessageType.All,
    replies: [],
    mentions: [],
    messages: [],
    sort: SortType.New,
    page: 1,
  };

  constructor(props: any, context: any) {
    super(props, context);

    this.state = this.emptyState;
    this.handleSortChange = this.handleSortChange.bind(this);

    this.subscription = WebSocketService.Instance.subject
      .pipe(retryWhen(errors => errors.pipe(delay(3000), take(10))))
      .subscribe(
        msg => this.parseMessage(msg),
        err => console.error(err),
        () => console.log('complete')
      );

    this.refetch();
  }

  componentWillUnmount() {
    this.subscription.unsubscribe();
  }

  componentDidMount() {
    document.title = `/u/${UserService.Instance.user.username} ${i18n.t(
      'inbox'
    )} - ${WebSocketService.Instance.site.name}`;
  }

  render() {
    return (
      <div className="container">
        <div className="row">
          <div className="col-12">
            <h5 className="mb-1">
              {i18n.t('inbox')}
              <small>
                <a
                  href={`/feeds/inbox/${UserService.Instance.auth}.xml`}
                  target="_blank"
                  title="RSS"
                >
                  <svg className="icon ml-2 text-muted small">
                    <use xlinkHref="#icon-rss">#</use>
                  </svg>
                </a>
              </small>
            </h5>
            {this.state.replies.length +
              this.state.mentions.length +
              this.state.messages.length >
              0 &&
              this.state.unreadOrAll == UnreadOrAll.Unread && (
                <ul className="list-inline mb-1 text-muted small font-weight-bold">
                  <li className="list-inline-item">
                    <span className="pointer" onClick={this.markAllAsRead}>
                      {i18n.t('mark_all_as_read')}
                    </span>
                  </li>
                </ul>
              )}
            {this.selects()}
            {this.state.messageType == MessageType.All && this.all()}
            {this.state.messageType == MessageType.Replies && this.replies()}
            {this.state.messageType == MessageType.Mentions && this.mentions()}
            {this.state.messageType == MessageType.Messages && this.messages()}
            {this.paginator()}
          </div>
        </div>
      </div>
    );
  }

  unreadOrAllRadios() {
    return (
      <div className="btn-group btn-group-toggle">
        <label
          className={`btn btn-sm btn-secondary pointer
            ${this.state.unreadOrAll == UnreadOrAll.Unread && 'active'}
          `}
        >
          <input
            type="radio"
            value={UnreadOrAll.Unread}
            checked={this.state.unreadOrAll == UnreadOrAll.Unread}
            onChange={this.handleUnreadOrAllChange}
          />
          {i18n.t('unread')}
        </label>
        <label
          className={`btn btn-sm btn-secondary pointer
            ${this.state.unreadOrAll == UnreadOrAll.All && 'active'}
          `}
        >
          <input
            type="radio"
            value={UnreadOrAll.All}
            checked={this.state.unreadOrAll == UnreadOrAll.All}
            onChange={this.handleUnreadOrAllChange}
          />
          {i18n.t('all')}
        </label>
      </div>
    );
  }

  messageTypeRadios() {
    return (
      <div className="btn-group btn-group-toggle">
        <label
          className={`btn btn-sm btn-secondary pointer btn-outline-light
            ${this.state.messageType == MessageType.All && 'active'}
          `}
        >
          <input
            type="radio"
            value={MessageType.All}
            checked={this.state.messageType == MessageType.All}
            onChange={this.handleMessageTypeChange}
          />
          {i18n.t('all')}
        </label>
        <label
          className={`btn btn-sm btn-secondary pointer btn-outline-light
            ${this.state.messageType == MessageType.Replies && 'active'}
          `}
        >
          <input
            type="radio"
            value={MessageType.Replies}
            checked={this.state.messageType == MessageType.Replies}
            onChange={this.handleMessageTypeChange}
          />
          {i18n.t('replies')}
        </label>
        <label
          className={`btn btn-sm btn-secondary pointer btn-outline-light
            ${this.state.messageType == MessageType.Mentions && 'active'}
          `}
        >
          <input
            type="radio"
            value={MessageType.Mentions}
            checked={this.state.messageType == MessageType.Mentions}
            onChange={this.handleMessageTypeChange}
          />
          {i18n.t('mentions')}
        </label>
        <label
          className={`btn btn-sm btn-secondary pointer btn-outline-light
            ${this.state.messageType == MessageType.Messages && 'active'}
          `}
        >
          <input
            type="radio"
            value={MessageType.Messages}
            checked={this.state.messageType == MessageType.Messages}
            onChange={this.handleMessageTypeChange}
          />
          {i18n.t('messages')}
        </label>
      </div>
    );
  }

  selects() {
    return (
      <div className="mb-2">
        <span className="mr-3">{this.unreadOrAllRadios()}</span>
        <span className="mr-3">{this.messageTypeRadios()}</span>
        <SortSelect
          sort={this.state.sort}
          onChange={this.handleSortChange}
          hideHot
        />
      </div>
    );
  }

  all() {
    let combined: Array<ReplyType> = [];

    combined.push(...this.state.replies);
    combined.push(...this.state.mentions);
    combined.push(...this.state.messages);

    // Sort it
    combined.sort((a, b) => b.published.localeCompare(a.published));

    return (
      <div>
        {combined.map(i =>
          isCommentType(i) ? (
            <CommentNodes
              nodes={[{ comment: i }]}
              noIndent
              markable
              showContext
            />
          ) : (
            <PrivateMessage privateMessage={i} />
          )
        )}
      </div>
    );
  }

  replies() {
    return (
      <div>
        <CommentNodes
          nodes={commentsToFlatNodes(this.state.replies)}
          noIndent
          markable
          showContext
        />
      </div>
    );
  }

  mentions() {
    return (
      <div>
        {this.state.mentions.map(mention => (
          <CommentNodes
            nodes={[{ comment: mention }]}
            noIndent
            markable
            showContext
          />
        ))}
      </div>
    );
  }

  messages() {
    return (
      <div>
        {this.state.messages.map(message => (
          <PrivateMessage privateMessage={message} />
        ))}
      </div>
    );
  }

  paginator() {
    return (
      <div className="mt-2">
        {this.state.page > 1 && (
          <button
            className="btn btn-sm btn-secondary mr-1"
            onClick={this.prevPage}
          >
            {i18n.t('prev')}
          </button>
        )}
        <button
          className="btn btn-sm btn-secondary"
          onClick={this.nextPage}
        >
          {i18n.t('next')}
        </button>
      </div>
    );
  }
  
  nextPage = () => {
    this.setState((state, props) => {
      return {page: state.page + 1};
    });
    this.refetch();
  }

  prevPage = () => {
    this.setState((state, props) => {
      return {page: state.page - 1};
    });
    this.refetch();
  }

  handleUnreadOrAllChange = (event: any) => {
    this.setState({
      unreadOrAll: Number(event.target.value),
      page: 1,
    });
    this.refetch();
  }

  handleMessageTypeChange = (event: any) => {
    this.setState({
      messageType: Number(event.target.value),
      page: 1,
    });

    this.refetch();
  }

  refetch() {
    let repliesForm: GetRepliesForm = {
      sort: SortType[this.state.sort],
      unread_only: this.state.unreadOrAll == UnreadOrAll.Unread,
      page: this.state.page,
      limit: fetchLimit,
    };
    WebSocketService.Instance.getReplies(repliesForm);

    let userMentionsForm: GetUserMentionsForm = {
      sort: SortType[this.state.sort],
      unread_only: this.state.unreadOrAll == UnreadOrAll.Unread,
      page: this.state.page,
      limit: fetchLimit,
    };
    WebSocketService.Instance.getUserMentions(userMentionsForm);

    let privateMessagesForm: GetPrivateMessagesForm = {
      unread_only: this.state.unreadOrAll == UnreadOrAll.Unread,
      page: this.state.page,
      limit: fetchLimit,
    };
    WebSocketService.Instance.getPrivateMessages(privateMessagesForm);
  }

  handleSortChange(val: SortType) {
    this.setState({
      sort: val,
      page: 1,
    });
    this.refetch();
  }

  markAllAsRead() {
    WebSocketService.Instance.markAllAsRead();
  }

  parseMessage(msg: WebSocketJsonResponse) {
    console.log(msg);
    let res = wsJsonToRes(msg);
    if (msg.error) {
      toast(i18n.t(msg.error), 'danger');
      return;
    } else if (msg.reconnect) {
      this.refetch();
    } else if (res.op == UserOperation.GetReplies) {
      let data = res.data as GetRepliesResponse;
      this.setState({ replies : data.replies });
      this.sendUnreadCount();
      window.scrollTo(0, 0);
      setupTippy();
    } else if (res.op == UserOperation.GetUserMentions) {
      let data = res.data as GetUserMentionsResponse;
      this.setState({ mentions : data.mentions });
      this.sendUnreadCount();
      window.scrollTo(0, 0);
      setupTippy();
    } else if (res.op == UserOperation.GetPrivateMessages) {
      let data = res.data as PrivateMessagesResponse;
      this.setState({ messages : data.messages });
      this.sendUnreadCount();
      window.scrollTo(0, 0);
      setupTippy();
    } else if (res.op == UserOperation.EditPrivateMessage) {
      let data = res.data as PrivateMessageResponse;
      let found: PrivateMessageI = this.state.messages.find(
        m => m.id === data.message.id
      );
      found.content = data.message.content;
      found.updated = data.message.updated;
      found.deleted = data.message.deleted;
      // If youre in the unread view, just remove it from the list
      if (this.state.unreadOrAll == UnreadOrAll.Unread && data.message.read) {
        this.setState({ messages: this.state.messages.filter(
          r => r.id !== data.message.id
        )});
      } else {
        let found = this.state.messages.find(c => c.id == data.message.id);
        found.read = data.message.read;
      }
      this.sendUnreadCount();
      window.scrollTo(0, 0);
      setupTippy();
    } else if (res.op == UserOperation.MarkAllAsRead) {
      this.sendUnreadCount();
      window.scrollTo(0, 0);
      this.setState({ replies: [], mentions: [], messages: []});
    } else if (res.op == UserOperation.EditComment) {
      let data = res.data as CommentResponse;
      editCommentRes(data, this.state.replies);

      // If youre in the unread view, just remove it from the list
      if (this.state.unreadOrAll == UnreadOrAll.Unread && data.comment.read) {
        this.setState({ replies: this.state.replies.filter(
          r => r.id !== data.comment.id
        )});
      } else {
        let found = this.state.replies.find(c => c.id == data.comment.id);
        found.read = data.comment.read;
      }
      this.sendUnreadCount();
      setupTippy();
    } else if (res.op == UserOperation.EditUserMention) {
      let data = res.data as UserMentionResponse;

      let found = this.state.mentions.find(c => c.id == data.mention.id);
      found.content = data.mention.content;
      found.updated = data.mention.updated;
      found.removed = data.mention.removed;
      found.deleted = data.mention.deleted;
      found.upvotes = data.mention.upvotes;
      found.downvotes = data.mention.downvotes;
      found.score = data.mention.score;

      // If youre in the unread view, just remove it from the list
      if (this.state.unreadOrAll == UnreadOrAll.Unread && data.mention.read) {
        this.setState({ mentions: this.state.mentions.filter(
          r => r.id !== data.mention.id
        )});
      } else {
        let found = this.state.mentions.find(c => c.id == data.mention.id);
        found.read = data.mention.read;
      }
      this.sendUnreadCount();
    } else if (res.op == UserOperation.CreateComment) {
      let data = res.data as CommentResponse;

      if (data.recipient_ids.includes(UserService.Instance.user.id)) {
        // this.state.replies.unshift(data.comment);
        let new_replies = [data.comment].concat(this.state.replies);
        this.setState({replies: new_replies});
      } else if (data.comment.creator_id == UserService.Instance.user.id) {
        toast(i18n.t('reply_sent'));
      }
    } else if (res.op == UserOperation.CreatePrivateMessage) {
      let data = res.data as PrivateMessageResponse;
      if (data.message.recipient_id == UserService.Instance.user.id) {
        // this.state.messages.unshift(data.message);
        let new_messages = [data.message].concat(this.state.messages);
        this.setState({messages: new_messages});
      }
    } else if (res.op == UserOperation.SaveComment) {
      let data = res.data as CommentResponse;
      saveCommentRes(data, this.state.replies);
      this.setState({replies: this.state.replies});
      setupTippy();
    } else if (res.op == UserOperation.CreateCommentLike) {
      let data = res.data as CommentResponse;
      createCommentLikeRes(data, this.state.replies);
      this.setState({replies: this.state.replies});
    }
  }

  sendUnreadCount() {
    let count =
      this.state.replies.filter(r => !r.read).length +
      this.state.mentions.filter(r => !r.read).length +
      this.state.messages.filter(
        r => !r.read && r.creator_id !== UserService.Instance.user.id
      ).length;
    UserService.Instance.user.unreadCount = count;
    UserService.Instance.sub.next({
      user: UserService.Instance.user,
    });
  }
}

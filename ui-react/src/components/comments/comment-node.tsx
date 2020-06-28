import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import {
  CommentNode as CommentNodeI,
  CommentLikeForm,
  CommentForm as CommentFormI,
  EditUserMentionForm,
  SaveCommentForm,
  BanFromCommunityForm,
  BanUserForm,
  CommunityUser,
  UserView,
  AddModToCommunityForm,
  AddAdminForm,
  TransferCommunityForm,
  TransferSiteForm,
  BanType,
  CommentSortType,
  SortType,
} from '../../interfaces';
import { WebSocketService, UserService } from '../../services';
import {
  mdToHtml,
  getUnixTime,
  canMod,
  isMod,
  setupTippy,
  colorList,
} from '../../utils';
import moment from 'moment';
import { MomentTime } from '../common/moment-time';
import { CommentForm } from './comment-form';
import { CommentNodes } from './comment-nodes';
import { UserListing } from '../users/user-listing';
import { i18n } from '../../i18next';

interface CommentNodeState {
  showReply: boolean;
  showEdit: boolean;
  showRemoveDialog: boolean;
  removeReason: string;
  showBanDialog: boolean;
  banReason: string;
  banExpires: string;
  banType: BanType;
  showConfirmTransferSite: boolean;
  showConfirmTransferCommunity: boolean;
  showConfirmAppointAsMod: boolean;
  showConfirmAppointAsAdmin: boolean;
  collapsed: boolean;
  viewSource: boolean;
  showAdvanced: boolean;
  my_vote: number;
  score: number;
  upvotes: number;
  downvotes: number;
  borderColor: string;
  readLoading: boolean;
  saveLoading: boolean;
}

interface CommentNodeProps {
  node: CommentNodeI;
  noIndent?: boolean;
  viewOnly?: boolean;
  locked?: boolean;
  markable?: boolean;
  showContext?: boolean;
  moderators: Array<CommunityUser>;
  admins: Array<UserView>;
  // TODO is this necessary, can't I get it from the node itself?
  postCreatorId?: number;
  showCommunity?: boolean;
  sort?: CommentSortType;
  sortType?: SortType;
}

export class CommentNode extends Component<CommentNodeProps, CommentNodeState> {

  state: CommentNodeState;

  private emptyState: CommentNodeState = {
    showReply: false,
    showEdit: false,
    showRemoveDialog: false,
    removeReason: null,
    showBanDialog: false,
    banReason: null,
    banExpires: null,
    banType: BanType.Community,
    collapsed: false,
    viewSource: false,
    showAdvanced: false,
    showConfirmTransferSite: false,
    showConfirmTransferCommunity: false,
    showConfirmAppointAsMod: false,
    showConfirmAppointAsAdmin: false,
    my_vote: this.props.node.comment.my_vote,
    score: this.props.node.comment.score,
    upvotes: this.props.node.comment.upvotes,
    downvotes: this.props.node.comment.downvotes,
    borderColor: this.props.node.comment.depth
      ? colorList[this.props.node.comment.depth % colorList.length]
      : colorList[0],
    readLoading: false,
    saveLoading: false,
  };

  constructor(props: any, context: any) {
    super(props, context);

    this.state = this.emptyState;
  }

  componentWillReceiveProps(nextProps: CommentNodeProps) {
    this.state.my_vote = nextProps.node.comment.my_vote;
    this.state.upvotes = nextProps.node.comment.upvotes;
    this.state.downvotes = nextProps.node.comment.downvotes;
    this.state.score = nextProps.node.comment.score;
    this.state.readLoading = false;
    this.state.saveLoading = false;
    this.setState(this.state);
  }

  render() {
    let node = this.props.node;
    let comment_style = (
      !this.props.noIndent && this.props.node.comment.parent_id)?
      { borderLeft : `border-left: 2px ${this.state.borderColor} solid !important` } :
      { };

    return (
      <div
        className={`comment ${
          node.comment.parent_id && !this.props.noIndent ? 'ml-1' : ''
        }`}
      >
        <div
          id={`comment-${node.comment.id}`}
          className={`details comment-node border-top border-light ${
            this.isCommentNew ? 'mark' : ''
          }`}
          style={comment_style}
        >
          <div
            className={`${
              !this.props.noIndent &&
              this.props.node.comment.parent_id &&
              'ml-2'
            }`}
          >
            <div className="d-flex flex-wrap align-items-center mb-1 mt-1 text-muted small">
              <span className="mr-2">
                <UserListing
                  user={{
                    name: node.comment.creator_name,
                    avatar: node.comment.creator_avatar,
                  }}
                />
              </span>
              {this.isMod && (
                <div className="badge badge-light d-none d-sm-inline mr-2">
                  {i18n.t('mod')}
                </div>
              )}
              {this.isAdmin && (
                <div className="badge badge-light d-none d-sm-inline mr-2">
                  {i18n.t('admin')}
                </div>
              )}
              {this.isPostCreator && (
                <div className="badge badge-light d-none d-sm-inline mr-2">
                  {i18n.t('creator')}
                </div>
              )}
              {(node.comment.banned_from_community || node.comment.banned) && (
                <div className="badge badge-danger mr-2">
                  {i18n.t('banned')}
                </div>
              )}
              {this.props.showCommunity && (
                <>
                  <span className="mx-1">{i18n.t('to')}</span>
                  <Link className="mr-2" to={`/c/${node.comment.community_name}`}>
                    {node.comment.community_name}
                  </Link>
                </>
              )}
              <div
                className="mr-lg-4 flex-grow-1 flex-lg-grow-0 unselectable pointer mx-2"
                onClick={this.handleCommentCollapse}
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
              <span
                className={`unselectable pointer ${this.scoreColor}`}
                onClick={() => this.handleCommentUpvote(node)}
                data-tippy-content={this.pointsTippy}
              >
                <svg className="icon icon-inline mr-1">
                  <use xlinkHref="#icon-zap"></use>
                </svg>
                <span className="mr-1">{this.state.score}</span>
              </span>
              <span className="mr-1">•</span>
              <span>
                <MomentTime data={node.comment} />
              </span>
            </div>
            {/* end of user row */}
            {this.state.showEdit && (
              <CommentForm
                node={node}
                edit
                onReplyCancel={this.handleReplyCancel}
                disabled={this.props.locked}
              />
            )}
            {!this.state.showEdit && !this.state.collapsed && (
              <div>
                {this.state.viewSource ? (
                  <pre>{this.commentUnlessRemoved}</pre>
                ) : (
                  <div
                    className="md-div"
                    dangerouslySetInnerHTML={mdToHtml(
                      this.commentUnlessRemoved
                    )}
                  />
                )}
                <div className="d-flex justify-content-between justify-content-lg-start flex-wrap text-muted font-weight-bold">
                  {this.props.showContext && this.linkBtn}
                  {this.props.markable && (
                    <button
                      className="btn btn-link btn-animate text-muted"
                      onClick={this.handleMarkRead}
                      data-tippy-content={
                        node.comment.read
                          ? i18n.t('mark_as_unread')
                          : i18n.t('mark_as_read')
                      }
                    >
                      {this.state.readLoading ? (
                        this.loadingIcon
                      ) : (
                        <svg
                          className={`icon icon-inline ${
                            node.comment.read && 'text-success'
                          }`}
                        >
                          <use xlinkHref="#icon-check"></use>
                        </svg>
                      )}
                    </button>
                  )}
                  {UserService.Instance.user && !this.props.viewOnly && (
                    <>
                      <button
                        className={`btn btn-link btn-animate ${
                          this.state.my_vote == 1 ? 'text-info' : 'text-muted'
                        }`}
                        onClick={() => this.handleCommentUpvote(node)}
                        data-tippy-content={i18n.t('upvote')}
                      >
                        <svg className="icon icon-inline">
                          <use xlinkHref="#icon-arrow-up"></use>
                        </svg>
                        {this.state.upvotes !== this.state.score && (
                          <span className="ml-1">{this.state.upvotes}</span>
                        )}
                      </button>
                      {WebSocketService.Instance.site.enable_downvotes && (
                        <button
                          className={`btn btn-link btn-animate ${
                            this.state.my_vote == -1
                              ? 'text-danger'
                              : 'text-muted'
                          }`}
                          onClick={() => this.handleCommentDownvote(node)}
                          data-tippy-content={i18n.t('downvote')}
                        >
                          <svg className="icon icon-inline">
                            <use xlinkHref="#icon-arrow-down"></use>
                          </svg>
                          {this.state.upvotes !== this.state.score && (
                            <span className="ml-1">{this.state.downvotes}</span>
                          )}
                        </button>
                      )}
                      <button
                        className="btn btn-link btn-animate text-muted"
                        onClick={this.handleSaveCommentClick}
                        data-tippy-content={
                          node.comment.saved ? i18n.t('unsave') : i18n.t('save')
                        }
                      >
                        {this.state.saveLoading ? (
                          this.loadingIcon
                        ) : (
                          <svg
                            className={`icon icon-inline ${
                              node.comment.saved && 'text-warning'
                            }`}
                          >
                            <use xlinkHref="#icon-star"></use>
                          </svg>
                        )}
                      </button>
                      <button
                        className="btn btn-link btn-animate text-muted"
                        onClick={this.handleReplyClick}
                        data-tippy-content={i18n.t('reply')}
                      >
                        <svg className="icon icon-inline">
                          <use xlinkHref="#icon-reply1"></use>
                        </svg>
                      </button>
                      {!this.state.showAdvanced ? (
                        <button
                          className="btn btn-link btn-animate text-muted"
                          onClick={this.handleShowAdvanced}
                          data-tippy-content={i18n.t('more')}
                        >
                          <svg className="icon icon-inline">
                            <use xlinkHref="#icon-more-vertical"></use>
                          </svg>
                        </button>
                      ) : (
                        <>
                          {!this.myComment && (
                            <button className="btn btn-link btn-animate">
                              <Link
                                className="text-muted"
                                to={`/create_private_message?recipient_id=${node.comment.creator_id}`}
                                title={i18n.t('message').toLowerCase()}
                              >
                                <svg className="icon">
                                  <use xlinkHref="#icon-mail"></use>
                                </svg>
                              </Link>
                            </button>
                          )}
                          {!this.props.showContext && this.linkBtn}
                          <button
                            className="btn btn-link btn-animate text-muted"
                            onClick={this.handleViewSource}
                            data-tippy-content={i18n.t('view_source')}
                          >
                            <svg
                              className={`icon icon-inline ${
                                this.state.viewSource && 'text-success'
                              }`}
                            >
                              <use xlinkHref="#icon-file-text"></use>
                            </svg>
                          </button>
                          {this.myComment && (
                            <>
                              <button
                                className="btn btn-link btn-animate text-muted"
                                onClick={this.handleEditClick}
                                data-tippy-content={i18n.t('edit')}
                              >
                                <svg className="icon icon-inline">
                                  <use xlinkHref="#icon-edit"></use>
                                </svg>
                              </button>
                              <button
                                className="btn btn-link btn-animate text-muted"
                                onClick={this.handleDeleteClick}
                                data-tippy-content={
                                  !node.comment.deleted
                                    ? i18n.t('delete')
                                    : i18n.t('restore')
                                }
                              >
                                <svg
                                  className={`icon icon-inline ${
                                    node.comment.deleted && 'text-danger'
                                  }`}
                                >
                                  <use xlinkHref="#icon-trash"></use>
                                </svg>
                              </button>
                            </>
                          )}
                          {/* Admins and mods can remove comments */}
                          {(this.canMod || this.canAdmin) && (
                            <>
                              {!node.comment.removed ? (
                                <button
                                  className="btn btn-link btn-animate text-muted"
                                  onClick={this.handleModRemoveShow}
                                >
                                  {i18n.t('remove')}
                                </button>
                              ) : (
                                <button
                                  className="btn btn-link btn-animate text-muted"
                                  onClick={this.handleModRemoveSubmit}
                                >
                                  {i18n.t('restore')}
                                </button>
                              )}
                            </>
                          )}
                          {/* Mods can ban from community, and appoint as mods to community */}
                          {this.canMod && (
                            <>
                              {!this.isMod &&
                                (!node.comment.banned_from_community ? (
                                  <button
                                    className="btn btn-link btn-animate text-muted"
                                    onClick={this.handleModBanFromCommunityShow}
                                  >
                                    {i18n.t('ban')}
                                  </button>
                                ) : (
                                  <button
                                    className="btn btn-link btn-animate text-muted"
                                    onClick={this.handleModBanFromCommunitySubmit}
                                  >
                                    {i18n.t('unban')}
                                  </button>
                                ))}
                              {!node.comment.banned_from_community &&
                                (!this.state.showConfirmAppointAsMod ? (
                                  <button
                                    className="btn btn-link btn-animate text-muted"
                                    onClick={this.handleShowConfirmAppointAsMod}
                                  >
                                    {this.isMod
                                      ? i18n.t('remove_as_mod')
                                      : i18n.t('appoint_as_mod')}
                                  </button>
                                ) : (
                                  <>
                                    <button className="btn btn-link btn-animate text-muted">
                                      {i18n.t('are_you_sure')}
                                    </button>
                                    <button
                                      className="btn btn-link btn-animate text-muted"
                                      onClick={this.handleAddModToCommunity}
                                    >
                                      {i18n.t('yes')}
                                    </button>
                                    <button
                                      className="btn btn-link btn-animate text-muted"
                                      onClick={this.handleCancelConfirmAppointAsMod}
                                    >
                                      {i18n.t('no')}
                                    </button>
                                  </>
                                ))}
                            </>
                          )}
                          {/* Community creators and admins can transfer community to another mod */}
                          {(this.amCommunityCreator || this.canAdmin) &&
                            this.isMod &&
                            (!this.state.showConfirmTransferCommunity ? (
                              <button
                                className="btn btn-link btn-animate text-muted"
                                onClick={this.handleShowConfirmTransferCommunity}
                              >
                                {i18n.t('transfer_community')}
                              </button>
                            ) : (
                              <>
                                <button className="btn btn-link btn-animate text-muted">
                                  {i18n.t('are_you_sure')}
                                </button>
                                <button
                                  className="btn btn-link btn-animate text-muted"
                                  onClick={this.handleTransferCommunity}
                                >
                                  {i18n.t('yes')}
                                </button>
                                <button
                                  className="btn btn-link btn-animate text-muted"
                                  onClick={this.handleCancelShowConfirmTransferCommunity}
                                >
                                  {i18n.t('no')}
                                </button>
                              </>
                            ))}
                          {/* Admins can ban from all, and appoint other admins */}
                          {this.canAdmin && (
                            <>
                              {!this.isAdmin &&
                                (!node.comment.banned ? (
                                  <button
                                    className="btn btn-link btn-animate text-muted"
                                    onClick={this.handleModBanShow}
                                  >
                                    {i18n.t('ban_from_site')}
                                  </button>
                                ) : (
                                  <button
                                    className="btn btn-link btn-animate text-muted"
                                    onClick={this.handleModBanSubmit}
                                  >
                                    {i18n.t('unban_from_site')}
                                  </button>
                                ))}
                              {!node.comment.banned &&
                                (!this.state.showConfirmAppointAsAdmin ? (
                                  <button
                                    className="btn btn-link btn-animate text-muted"
                                    onClick={this.handleShowConfirmAppointAsAdmin}
                                  >
                                    {this.isAdmin
                                      ? i18n.t('remove_as_admin')
                                      : i18n.t('appoint_as_admin')}
                                  </button>
                                ) : (
                                  <>
                                    <button className="btn btn-link btn-animate text-muted">
                                      {i18n.t('are_you_sure')}
                                    </button>
                                    <button
                                      className="btn btn-link btn-animate text-muted"
                                      onClick={this.handleAddAdmin}
                                    >
                                      {i18n.t('yes')}
                                    </button>
                                    <button
                                      className="btn btn-link btn-animate text-muted"
                                      onClick={this.handleCancelConfirmAppointAsAdmin}
                                    >
                                      {i18n.t('no')}
                                    </button>
                                  </>
                                ))}
                            </>
                          )}
                          {/* Site Creator can transfer to another admin */}
                          {this.amSiteCreator &&
                            this.isAdmin &&
                            (!this.state.showConfirmTransferSite ? (
                              <button
                                className="btn btn-link btn-animate text-muted"
                                onClick={this.handleShowConfirmTransferSite}
                              >
                                {i18n.t('transfer_site')}
                              </button>
                            ) : (
                              <>
                                <button className="btn btn-link btn-animate text-muted">
                                  {i18n.t('are_you_sure')}
                                </button>
                                <button
                                  className="btn btn-link btn-animate text-muted"
                                  onClick={this.handleTransferSite}
                                >
                                  {i18n.t('yes')}
                                </button>
                                <button
                                  className="btn btn-link btn-animate text-muted"
                                  onClick={this.handleCancelShowConfirmTransferSite}
                                >
                                  {i18n.t('no')}
                                </button>
                              </>
                            ))}
                        </>
                      )}
                    </>
                  )}
                </div>
                {/* end of button group */}
              </div>
            )}
          </div>
        </div>
        {/* end of details */}
        {this.state.showRemoveDialog && (
          <form
            className="form-inline"
            onSubmit={this.handleModRemoveSubmit}
          >
            <input
              type="text"
              className="form-control mr-2"
              placeholder={i18n.t('reason')}
              value={this.state.removeReason}
              onInput={this.handleModRemoveReasonChange}
            />
            <button type="submit" className="btn btn-secondary">
              {i18n.t('remove_comment')}
            </button>
          </form>
        )}
        {this.state.showBanDialog && (
          <form onSubmit={this.handleModBanBothSubmit}>
            <div className="form-group row">
              <label className="col-form-label">{i18n.t('reason')}</label>
              <input
                type="text"
                className="form-control mr-2"
                placeholder={i18n.t('reason')}
                value={this.state.banReason}
                onInput={this.handleModBanReasonChange}
              />
            </div>
            {/* TODO hold off on expires until later */}
            {/* <div className="form-group row"> */}
            {/*   <label className="col-form-label">Expires</label> */}
            {/*   <input type="date" className="form-control mr-2" placeholder={i18n.t('expires')} value={this.state.banExpires} onInput={this.handleModBanExpiresChange} /> */}
            {/* </div> */}
            <div className="form-group row">
              <button type="submit" className="btn btn-secondary">
                {i18n.t('ban')} {node.comment.creator_name}
              </button>
            </div>
          </form>
        )}
        {this.state.showReply && (
          <CommentForm
            node={node}
            onReplyCancel={this.handleReplyCancel}
            disabled={this.props.locked}
          />
        )}
        {node.children && !this.state.collapsed && (
          <CommentNodes
            nodes={node.children}
            locked={this.props.locked}
            moderators={this.props.moderators}
            admins={this.props.admins}
            postCreatorId={this.props.postCreatorId}
            sort={this.props.sort}
            sortType={this.props.sortType}
          />
        )}
        {/* A collapsed clearfix */}
        {this.state.collapsed && <div className="row col-12"></div>}
      </div>
    );
  }

  get linkBtn() {
    let node = this.props.node;
    return (
      <Link
        className="btn btn-link btn-animate text-muted"
        to={`/post/${node.comment.post_id}/comment/${node.comment.id}`}
        title={this.props.showContext ? i18n.t('show_context') : i18n.t('link')}
      >
        <svg className="icon icon-inline">
          <use xlinkHref="#icon-link"></use>
        </svg>
      </Link>
    );
  }

  get loadingIcon() {
    return (
      <svg className="icon icon-spinner spin">
        <use xlinkHref="#icon-spinner"></use>
      </svg>
    );
  }

  get myComment(): boolean {
    return (
      UserService.Instance.user &&
      this.props.node.comment.creator_id == UserService.Instance.user.id
    );
  }

  get isMod(): boolean {
    return (
      this.props.moderators &&
      isMod(
        this.props.moderators.map(m => m.user_id),
        this.props.node.comment.creator_id
      )
    );
  }

  get isAdmin(): boolean {
    return (
      this.props.admins &&
      isMod(
        this.props.admins.map(a => a.id),
        this.props.node.comment.creator_id
      )
    );
  }

  get isPostCreator(): boolean {
    return this.props.node.comment.creator_id == this.props.postCreatorId;
  }

  get canMod(): boolean {
    if (this.props.admins && this.props.moderators) {
      let adminsThenMods = this.props.admins
        .map(a => a.id)
        .concat(this.props.moderators.map(m => m.user_id));

      return canMod(
        UserService.Instance.user,
        adminsThenMods,
        this.props.node.comment.creator_id
      );
    } else {
      return false;
    }
  }

  get canAdmin(): boolean {
    return (
      this.props.admins &&
      canMod(
        UserService.Instance.user,
        this.props.admins.map(a => a.id),
        this.props.node.comment.creator_id
      )
    );
  }

  get amCommunityCreator(): boolean {
    return (
      this.props.moderators &&
      UserService.Instance.user &&
      this.props.node.comment.creator_id != UserService.Instance.user.id &&
      UserService.Instance.user.id == this.props.moderators[0].user_id
    );
  }

  get amSiteCreator(): boolean {
    return (
      this.props.admins &&
      UserService.Instance.user &&
      this.props.node.comment.creator_id != UserService.Instance.user.id &&
      UserService.Instance.user.id == this.props.admins[0].id
    );
  }

  get commentUnlessRemoved(): string {
    let node = this.props.node;
    return node.comment.removed
      ? `*${i18n.t('removed')}*`
      : node.comment.deleted
      ? `*${i18n.t('deleted')}*`
      : node.comment.content;
  }

  handleReplyClick = () => {
    this.state.showReply = true;
    this.setState(this.state);
  }

  handleEditClick = () => {
    this.state.showEdit = true;
    this.setState(this.state);
  }

  handleDeleteClick = () => {
    let deleteForm: CommentFormI = {
      content: this.props.node.comment.content,
      edit_id: this.props.node.comment.id,
      creator_id: this.props.node.comment.creator_id,
      post_id: this.props.node.comment.post_id,
      parent_id: this.props.node.comment.parent_id,
      deleted: !this.props.node.comment.deleted,
      auth: null,
    };
    WebSocketService.Instance.editComment(deleteForm);
  }

  handleSaveCommentClick = () => {
    let saved =
      this.props.node.comment.saved == undefined
        ? true
        : !this.props.node.comment.saved;
    let form: SaveCommentForm = {
      comment_id: this.props.node.comment.id,
      save: saved,
    };

    WebSocketService.Instance.saveComment(form);

    this.state.saveLoading = true;
    this.setState(this.state);
  }

  handleReplyCancel = () => {
    this.state.showReply = false;
    this.state.showEdit = false;
    this.setState(this.state);
  }

  handleCommentUpvote = (i: CommentNodeI) => {
    let new_vote = this.state.my_vote == 1 ? 0 : 1;

    if (this.state.my_vote == 1) {
      this.state.score--;
      this.state.upvotes--;
    } else if (this.state.my_vote == -1) {
      this.state.downvotes--;
      this.state.upvotes++;
      this.state.score += 2;
    } else {
      this.state.upvotes++;
      this.state.score++;
    }

    this.state.my_vote = new_vote;

    let form: CommentLikeForm = {
      comment_id: i.comment.id,
      post_id: i.comment.post_id,
      score: this.state.my_vote,
    };

    WebSocketService.Instance.likeComment(form);
    this.setState(this.state);
    setupTippy();
  }

  handleCommentDownvote(i: CommentNodeI) {
    let new_vote = this.state.my_vote == -1 ? 0 : -1;

    if (this.state.my_vote == 1) {
      this.state.score -= 2;
      this.state.upvotes--;
      this.state.downvotes++;
    } else if (this.state.my_vote == -1) {
      this.state.downvotes--;
      this.state.score++;
    } else {
      this.state.downvotes++;
      this.state.score--;
    }

    this.state.my_vote = new_vote;

    let form: CommentLikeForm = {
      comment_id: i.comment.id,
      post_id: i.comment.post_id,
      score: this.state.my_vote,
    };

    WebSocketService.Instance.likeComment(form);
    this.setState(this.state);
    setupTippy();
  }

  handleModRemoveShow = () => {
    this.state.showRemoveDialog = true;
    this.setState(this.state);
  }

  handleModRemoveReasonChange = (event: any) => {
    this.state.removeReason = event.target.value;
    this.setState(this.state);
  }

  handleModRemoveSubmit = (event: any) => {
    event.preventDefault();
    let form: CommentFormI = {
      content: this.props.node.comment.content,
      edit_id: this.props.node.comment.id,
      creator_id: this.props.node.comment.creator_id,
      post_id: this.props.node.comment.post_id,
      parent_id: this.props.node.comment.parent_id,
      removed: !this.props.node.comment.removed,
      reason: this.state.removeReason,
      auth: null,
    };
    WebSocketService.Instance.editComment(form);

    this.state.showRemoveDialog = false;
    this.setState(this.state);
  }

  handleMarkRead = () => {
    // if it has a user_mention_id field, then its a mention
    if (this.props.node.comment.user_mention_id) {
      let form: EditUserMentionForm = {
        user_mention_id: this.props.node.comment.user_mention_id,
        read: !this.props.node.comment.read,
      };
      WebSocketService.Instance.editUserMention(form);
    } else {
      let form: CommentFormI = {
        content: this.props.node.comment.content,
        edit_id: this.props.node.comment.id,
        creator_id: this.props.node.comment.creator_id,
        post_id: this.props.node.comment.post_id,
        parent_id: this.props.node.comment.parent_id,
        read: !this.props.node.comment.read,
        auth: null,
      };
      WebSocketService.Instance.editComment(form);
    }

    this.state.readLoading = true;
    this.setState(this.state);
  }

  handleModBanFromCommunityShow = () => {
    this.state.showBanDialog = !this.state.showBanDialog;
    this.state.banType = BanType.Community;
    this.setState(this.state);
  }

  handleModBanShow = () => {
    this.state.showBanDialog = !this.state.showBanDialog;
    this.state.banType = BanType.Site;
    this.setState(this.state);
  }

  handleModBanReasonChange = (event: any) => {
    this.state.banReason = event.target.value;
    this.setState(this.state);
  }

  handleModBanExpiresChange = (event: any) => {
    this.state.banExpires = event.target.value;
    this.setState(this.state);
  }

  handleModBanFromCommunitySubmit = (event: any) => {
    this.state.banType = BanType.Community;
    this.setState(this.state);
    this.handleModBanBothSubmit(event);
  }

  handleModBanSubmit = (event: any) => {
    this.state.banType = BanType.Site;
    this.setState(this.state);
    this.handleModBanBothSubmit(event);
  }

  handleModBanBothSubmit = (event: any) => {
    event.preventDefault();

    if (this.state.banType == BanType.Community) {
      let form: BanFromCommunityForm = {
        user_id: this.props.node.comment.creator_id,
        community_id: this.props.node.comment.community_id,
        ban: !this.props.node.comment.banned_from_community,
        reason: this.state.banReason,
        expires: getUnixTime(this.state.banExpires),
      };
      WebSocketService.Instance.banFromCommunity(form);
    } else {
      let form: BanUserForm = {
        user_id: this.props.node.comment.creator_id,
        ban: !this.props.node.comment.banned,
        reason: this.state.banReason,
        expires: getUnixTime(this.state.banExpires),
      };
      WebSocketService.Instance.banUser(form);
    }

    this.state.showBanDialog = false;
    this.setState(this.state);
  }

  handleShowConfirmAppointAsMod = () => {
    this.state.showConfirmAppointAsMod = true;
    this.setState(this.state);
  }

  handleCancelConfirmAppointAsMod = () => {
    this.state.showConfirmAppointAsMod = false;
    this.setState(this.state);
  }

  handleAddModToCommunity = () => {
    let form: AddModToCommunityForm = {
      user_id: this.props.node.comment.creator_id,
      community_id: this.props.node.comment.community_id,
      added: !this.isMod,
    };
    WebSocketService.Instance.addModToCommunity(form);
    this.state.showConfirmAppointAsMod = false;
    this.setState(this.state);
  }

  handleShowConfirmAppointAsAdmin = () => {
    this.state.showConfirmAppointAsAdmin = true;
    this.setState(this.state);
  }

  handleCancelConfirmAppointAsAdmin = () => {
    this.state.showConfirmAppointAsAdmin = false;
    this.setState(this.state);
  }

  handleAddAdmin = () => {
    let form: AddAdminForm = {
      user_id: this.props.node.comment.creator_id,
      added: !this.isAdmin,
    };
    WebSocketService.Instance.addAdmin(form);
    this.state.showConfirmAppointAsAdmin = false;
    this.setState(this.state);
  }

  handleShowConfirmTransferCommunity = () => {
    this.state.showConfirmTransferCommunity = true;
    this.setState(this.state);
  }

  handleCancelShowConfirmTransferCommunity = () => {
    this.state.showConfirmTransferCommunity = false;
    this.setState(this.state);
  }

  handleTransferCommunity = () => {
    let form: TransferCommunityForm = {
      community_id: this.props.node.comment.community_id,
      user_id: this.props.node.comment.creator_id,
    };
    WebSocketService.Instance.transferCommunity(form);
    this.state.showConfirmTransferCommunity = false;
    this.setState(this.state);
  }

  handleShowConfirmTransferSite = () => {
    this.state.showConfirmTransferSite = true;
    this.setState(this.state);
  }

  handleCancelShowConfirmTransferSite = () => {
    this.state.showConfirmTransferSite = false;
    this.setState(this.state);
  }

  handleTransferSite = () => {
    let form: TransferSiteForm = {
      user_id: this.props.node.comment.creator_id,
    };
    WebSocketService.Instance.transferSite(form);
    this.state.showConfirmTransferSite = false;
    this.setState(this.state);
  }

  get isCommentNew(): boolean {
    let now = moment.utc().subtract(10, 'minutes');
    let then = moment.utc(this.props.node.comment.published);
    return now.isBefore(then);
  }

  handleCommentCollapse = () => {
    this.state.collapsed = !this.state.collapsed;
    this.setState(this.state);
  }

  handleViewSource = () => {
    this.state.viewSource = !this.state.viewSource;
    this.setState(this.state);
  }

  handleShowAdvanced = () => {
    this.state.showAdvanced = !this.state.showAdvanced;
    this.setState(this.state);
    setupTippy();
  }

  get scoreColor() {
    if (this.state.my_vote == 1) {
      return 'text-info';
    } else if (this.state.my_vote == -1) {
      return 'text-danger';
    } else {
      return 'text-muted';
    }
  }

  get pointsTippy(): string {
    let points = i18n.t('number_of_points', {
      count: this.state.score,
    });

    let upvotes = i18n.t('number_of_upvotes', {
      count: this.state.upvotes,
    });

    let downvotes = i18n.t('number_of_downvotes', {
      count: this.state.downvotes,
    });

    return `${points} • ${upvotes} • ${downvotes}`;
  }
}

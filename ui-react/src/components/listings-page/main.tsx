import React, { Component } from 'react';

import { Subscription } from 'rxjs';
import { retryWhen, delay, take } from 'rxjs/operators';

import {
  UserOperation,
  CommunityUser,
  GetFollowedCommunitiesResponse,
  ListCommunitiesForm,
  ListCommunitiesResponse,
  Community,
  SortType,
  GetSiteResponse,
  ListingType,
  DataType,
  SiteResponse,
  GetPostsResponse,
  PostResponse,
  Post,
  GetPostsForm,
  Comment,
  GetCommentsForm,
  GetCommentsResponse,
  CommentResponse,
  AddAdminResponse,
  BanUserResponse,
  WebSocketJsonResponse,
} from '../../interfaces';

import { WebSocketService, UserService } from '../../services';

import { Sidebar } from './main-sidebar';
import { PostListings } from '../posts/post-listings';
import { CommentNodes } from '../comments/comment-nodes';
import { SortSelect } from './sort-select';
import { ListingTypeSelect } from './listing-type-select';
import { DataTypeSelect } from './data-type-select';

import {
  wsJsonToRes,
  fetchLimit,
  toast,
  getListingTypeFromProps,
  getPageFromProps,
  getSortTypeFromProps,
  getDataTypeFromProps,
  editCommentRes,
  saveCommentRes,
  createCommentLikeRes,
  createPostLikeFindRes,
  editPostFindRes,
  commentsToFlatNodes,
  setupTippy,
} from '../../utils';

// Config
import { i18n } from '../../i18next';


interface MainState {
  subscribedCommunities: Array<CommunityUser>;
  trendingCommunities: Array<Community>;
  siteRes: GetSiteResponse;
  showEditSite: boolean;
  loading: boolean;
  posts: Array<Post>;
  comments: Array<Comment>;
  listingType: ListingType;
  dataType: DataType;
  sort: SortType;
  page: number;
}

/**
 * Landing page showing post listings when you're not
 * in a specific commnity.
 */
export class Main extends Component<any, MainState> {

  private emptyState: MainState = {
    subscribedCommunities: [],
    trendingCommunities: [],
    siteRes: {
      site: {
        id: null,
        name: null,
        creator_id: null,
        creator_name: null,
        published: null,
        number_of_users: null,
        number_of_posts: null,
        number_of_comments: null,
        number_of_communities: null,
        enable_downvotes: null,
        open_registration: null,
        enable_nsfw: null,
      },
      admins: [],
      banned: [],
      online: null,
    },
    showEditSite: false,
    loading: true,
    posts: [],
    comments: [],
    listingType: getListingTypeFromProps(this.props),
    dataType: getDataTypeFromProps(this.props),
    sort: getSortTypeFromProps(this.props),
    page: getPageFromProps(this.props),
  };

  private subscription: Subscription;


  /**
   * Create the main (front page) component.
   * 
   * @param props 
   * @param context 
   */
  constructor(props: any, context: any) {
    super(props, context);
    this.state = this.emptyState

    this.handleSortChange = this.handleSortChange.bind(this);
    this.handleListingTypeChange = this.handleListingTypeChange.bind(this);
    this.handleDataTypeChange = this.handleDataTypeChange.bind(this);

    this.subscription = WebSocketService.Instance.subject
      .pipe(retryWhen(errors => errors.pipe(delay(3000), take(10))))
      .subscribe(
        msg => this.parseMessage(msg),
        err => console.error(err),
        () => console.log('complete')
      );

    WebSocketService.Instance.getSite();

    if (UserService.Instance.user) {
      WebSocketService.Instance.getFollowedCommunities();
    }

    let listCommunitiesForm: ListCommunitiesForm = {
      sort: SortType[SortType.Hot],
      limit: 6,
    };

    WebSocketService.Instance.listCommunities(listCommunitiesForm);

    this.fetchData();
  }


  /**
   * @override React.Component.render
   */
  render() {
    return (
      <div className="container">
        <div className="row">
          <main role="main" className="col-12 col-md-8">
            {this.posts()}
          </main>
          <aside className="col-12 col-md-4">
            <Sidebar
              user={UserService.Instance.user}
              subscribedCommunities={this.state.subscribedCommunities}
              trendingCommunities={this.state.trendingCommunities}
              loading={this.state.loading}
              showEditSite={this.state.showEditSite}
              siteRes={this.state.siteRes}
              canAdmin={this.canAdmin}
              handleEditCancel={this.handleEditCancel}
              handleEditClick={this.handleEditClick} />
          </aside>
        </div>
      </div>
    );
  }


  /**
   * @override React.Component.componentWillUnmount
   */
  componentWillUnmount() {
    this.subscription.unsubscribe();
  }


  /**
   * Necessary for back button to work
   * 
   * @override componentWillReceiveProps
   */
  componentWillReceiveProps(nextProps: any) {
    if (
      nextProps.history.action == 'POP' ||
      nextProps.history.action == 'PUSH'
    ) {
      this.setState({
        listingType: getListingTypeFromProps(nextProps),
        dataType: getDataTypeFromProps(nextProps),
        sort: getSortTypeFromProps(nextProps),
        page: getPageFromProps(nextProps),
      })
      this.fetchData();
    }
  }

  /**
   * Update url displayed based on pagination/sorting changes.
   */
  updateUrl() {
    let listingTypeStr = ListingType[this.state.listingType].toLowerCase();
    let dataTypeStr = DataType[this.state.dataType].toLowerCase();
    let sortStr = SortType[this.state.sort].toLowerCase();
    this.props.history.push(
      `/home/data_type/${dataTypeStr}/listing_type/${listingTypeStr}/sort/${sortStr}/page/${this.state.page}`
    );
  }

  posts() {
    return (
      <div className="main-content-wrapper">
        {this.selects()}
        {this.state.loading ? (
          <h5>
            <svg className="icon icon-spinner spin">
              <use xlinkHref="#icon-spinner"></use>
            </svg>
          </h5>
        ) : (
          <div>
            {this.listings()}
            {this.paginator()}
          </div>
        )}
      </div>
    );
  }

  /**
   * Lists either posts or comments depending on selected button.
   */
  listings() {
    return this.state.dataType == DataType.Post ? (
      <PostListings
        posts={this.state.posts}
        showCommunity
        removeDuplicates
        sort={this.state.sort}
      />
    ) : (
      <CommentNodes
        nodes={commentsToFlatNodes(this.state.comments)}
        noIndent
        showCommunity
        sortType={this.state.sort}
        showContext
      />
    );
  }

  /**
   * Selection buttons/dropdowns for displaying/sorting posts.
   * 
   * E.g. (Posts | comments), (subscribed | all), (hot | new | top)
   */
  selects() {
    return (
      <div className="mb-3">
        <span className="mr-3">
          <DataTypeSelect
            type_={this.state.dataType}
            onChange={this.handleDataTypeChange}
          />
        </span>
        <span className="mr-3">
          <ListingTypeSelect
            type_={this.state.listingType}
            onChange={this.handleListingTypeChange}
          />
        </span>
        <span className="mr-2">
          <SortSelect sort={this.state.sort} onChange={this.handleSortChange} />
        </span>
        {this.state.listingType == ListingType.All && (
          <a
            href={`/feeds/all.xml?sort=${SortType[this.state.sort]}`}
            target="_blank"
            title="RSS"
          >
            <svg className="icon text-muted small">
              <use xlinkHref="#icon-rss">#</use>
            </svg>
          </a>
        )}
        {UserService.Instance.user &&
          this.state.listingType == ListingType.Subscribed && (
            <a
              href={`/feeds/front/${UserService.Instance.auth}.xml?sort=${
                SortType[this.state.sort]
              }`}
              target="_blank"
              title="RSS"
            >
              <svg className="icon text-muted small">
                <use xlinkHref="#icon-rss">#</use>
              </svg>
            </a>
          )}
      </div>
    );
  }

  paginator() {
    return (
      <div className="my-2">
        {this.state.page > 1 && (
          <button
            className="btn btn-sm btn-secondary mr-1"
            onClick={this.prevPage}
          >
            {i18n.t('prev')}
          </button>
        )}
        {this.state.posts.length == fetchLimit && (
          <button
            className="btn btn-sm btn-secondary"
            onClick={this.nextPage}
          >
            {i18n.t('next')}
          </button>
        )}
      </div>
    );
  }

  canAdmin: () => boolean = () => {
    return (
      UserService.Instance.user &&
      this.state.siteRes.admins
        .map(a => a.id)
        .includes(UserService.Instance.user.id)
    );
  }


  handleEditClick = () => {
    this.setState({ showEditSite: true });
  }

  handleEditCancel = () => {
    this.setState({ showEditSite: false });
  }

  nextPage = () => {
    this.setState({ page:this.state.page+1, loading:true })
    this.updateUrl();
    this.fetchData();
    window.scrollTo(0, 0);
  }

  prevPage = () => {
    this.setState({ page:this.state.page-1, loading:true })
    this.updateUrl();
    this.fetchData();
    window.scrollTo(0, 0);
  }

  handleSortChange = (val: SortType) => {
    this.setState({
      sort: val,
      page: 1,
      loading: true,
    });
    this.updateUrl();
    this.fetchData();
    window.scrollTo(0, 0);
  }

  handleListingTypeChange = (val: ListingType) => {
    this.setState({
      listingType: val,
      page: 1,
      loading: true,
    });
    this.updateUrl();
    this.fetchData();
    window.scrollTo(0, 0);
  }

  handleDataTypeChange = (val: DataType) => {
    this.setState({
      dataType: val,
      page: 1,
      loading: true,
    });
    this.updateUrl();
    this.fetchData();
    window.scrollTo(0, 0);
  }

  fetchData() {
    if (this.state.dataType == DataType.Post) {
      let getPostsForm: GetPostsForm = {
        page: this.state.page,
        limit: fetchLimit,
        sort: SortType[this.state.sort],
        type_: ListingType[this.state.listingType],
      };
      WebSocketService.Instance.getPosts(getPostsForm);
    } else {
      let getCommentsForm: GetCommentsForm = {
        page: this.state.page,
        limit: fetchLimit,
        sort: SortType[this.state.sort],
        type_: ListingType[this.state.listingType],
      };
      WebSocketService.Instance.getComments(getCommentsForm);
    }
  }

  parseMessage = (msg: WebSocketJsonResponse) => {
    console.log(msg);
    let res = wsJsonToRes(msg);
    if (msg.error) {
      toast(i18n.t(msg.error), 'danger');
      return;
    } else if (msg.reconnect) {
      this.fetchData();

    } else if (res.op == UserOperation.GetFollowedCommunities) {
      let data = res.data as GetFollowedCommunitiesResponse;
      this.setState({
        subscribedCommunities: data.communities
      });

    } else if (res.op == UserOperation.ListCommunities) {
      let data = res.data as ListCommunitiesResponse;
      this.setState({
        trendingCommunities: data.communities
      });

    } else if (res.op == UserOperation.GetSite) {
      let data = res.data as GetSiteResponse;

      // This means it hasn't been set up yet
      if (!data.site) {
        this.context.router.history.push('/setup');
      }
      this.setState({
        siteRes : {
          ...this.state.siteRes,
          admins: data.admins,
          site: data.site,
          banned: data.banned,
          online: data.online,
        }
      });
      document.title = `${WebSocketService.Instance.site.name}`;

    } else if (res.op == UserOperation.EditSite) {
      let data = res.data as SiteResponse;
      this.setState({
        showEditSite: false,
        siteRes : {
          ...this.state.siteRes,
          site: data.site,
        }
      });
      toast(i18n.t('site_saved'));

    } else if (res.op == UserOperation.GetPosts) {
      let data = res.data as GetPostsResponse;
      this.setState({
        posts: data.posts,
        loading: false,
      });
      setupTippy();

      // TODO: finish updates setstate or use reduced
      //  - https://reactjs.org/docs/hooks-reference.html#usereducer
    } else if (res.op == UserOperation.CreatePost) {
      let data = res.data as PostResponse;
      let posts = this.state.posts;

      // If you're on subscribed, only push it if you're subscribed.
      if (this.state.listingType == ListingType.Subscribed) {
        if (
          this.state.subscribedCommunities
            .map(c => c.community_id)
            .includes(data.post.community_id)
        ) {
          posts.unshift(data.post);
        }
      } else {
        // NSFW posts
        let nsfw = data.post.nsfw || data.post.community_nsfw;

        // Don't push the post if its nsfw, and don't have that setting on
        if (
          !nsfw ||
          (nsfw &&
            UserService.Instance.user &&
            UserService.Instance.user.show_nsfw)
        ) {
          posts.unshift(data.post);
        }
      }
      this.setState({
        posts: posts,
      });

    } else if (res.op == UserOperation.EditPost) {
      let data = res.data as PostResponse;
      editPostFindRes(data, this.state.posts);
      this.setState({
        posts: this.state.posts,
      });

    } else if (res.op == UserOperation.CreatePostLike) {
      let data = res.data as PostResponse;
      createPostLikeFindRes(data, this.state.posts);
      this.setState({
        posts: this.state.posts,
      });

    } else if (res.op == UserOperation.AddAdmin) {
      let data = res.data as AddAdminResponse;
      this.setState({
        siteRes : {
          ...this.state.siteRes,
          admins: data.admins,
        }
      });

    } else if (res.op == UserOperation.BanUser) {
      let data = res.data as BanUserResponse;
      let found = this.state.siteRes.banned.find(u => (u.id = data.user.id));

      // Remove the banned if its found in the list, and the action is an unban
      if (found && !data.banned) {
         this.state.siteRes.banned = this.state.siteRes.banned.filter(
          i => i.id !== data.user.id
        );
      } else {
        this.state.siteRes.banned.push(data.user);
      }

      this.state.posts
        .filter(p => p.creator_id == data.user.id)
        .forEach(p => (p.banned = data.banned));

      this.setState({
        posts: this.state.posts,
        siteRes: this.state.siteRes,
      });

    } else if (res.op == UserOperation.GetComments) {
      let data = res.data as GetCommentsResponse;
      this.setState({
        comments: data.comments,
        loading: false,
      });

    } else if (res.op == UserOperation.EditComment) {
      let data = res.data as CommentResponse;
      editCommentRes(data, this.state.comments);
      this.setState({
        comments: this.state.comments,
      });

    } else if (res.op == UserOperation.CreateComment) {
      let data = res.data as CommentResponse;

      // Necessary since it might be a user reply
      if (data.recipient_ids.length == 0) {
        // If you're on subscribed, only push it if you're subscribed.
        if (this.state.listingType == ListingType.Subscribed) {
          if (
            this.state.subscribedCommunities
              .map(c => c.community_id)
              .includes(data.comment.community_id)
          ) {
            this.state.comments.unshift(data.comment);
          }
        } else {
          this.state.comments.unshift(data.comment);
        }
        this.setState({
          comments: this.state.comments,
        })
      }

    } else if (res.op == UserOperation.SaveComment) {
      let data = res.data as CommentResponse;
      saveCommentRes(data, this.state.comments);
      this.setState({
        comments: this.state.comments,
      })

    } else if (res.op == UserOperation.CreateCommentLike) {
      let data = res.data as CommentResponse;
      createCommentLikeRes(data, this.state.comments);
      this.setState({
        comments: this.state.comments,
      })
    }
  }

}


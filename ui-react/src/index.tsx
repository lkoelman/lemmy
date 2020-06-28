// React
import React, { Component } from "react";
import ReactDOM from "react-dom";
import {
    BrowserRouter,
    Switch,
    Route
} from "react-router-dom";

import { GraphQlService } from './services';
import { ApolloProvider } from 'react-apollo'
import { ApolloClient } from 'apollo-boost'

// Config
import { i18n } from './i18next';
import { Trans } from 'react-i18next';

// Components
import { Main } from './components/listings-page/main';
import { Navbar } from './components/common/navbar';
import { Footer } from './components/common/footer';
import { Login } from './components/users/login';
import { CreatePost } from './components/posts/create-post';
import { CreateCommunity } from './components/communities/create-community';
import { CreatePrivateMessage } from './components/messaging/create-private-message';
import { PasswordChange } from './components/users/password_change';
import { Post } from './components/posts/post';
import { Community } from './components/listings-page/community';
import { Communities } from './components/communities/communities';
import { User } from './components/users/user';
import { Modlog } from './components/common/modlog';
import { Setup } from './components/users/setup';
import { AdminSettings } from './components/communities/admin-settings';
import { Inbox } from './components/messaging/inbox';
import { Search } from './components/common/search';
import { Sponsors } from './components/common/sponsors';
import { Symbols } from './components/common/symbols';

class Index extends Component<any, any> {

  private client: ApolloClient<any>;

  constructor(props: any, context: any) {
    super(props, context);

    this.client = GraphQlService.Instance.client;
  }

  render() {
    return (
      <Trans>
        <ApolloProvider client={this.client}>
        <BrowserRouter>
          <div>
            <Navbar />
            <div className="mt-4 p-0 fl-1">
              <Switch>
                <Route exact path={`/`} component={Main} />
                <Route
                  path={`/home/data_type/:data_type/listing_type/:listing_type/sort/:sort/page/:page`}
                  component={Main}
                />
                <Route path={`/login`} component={Login} />
                <Route path={`/create_post`} component={CreatePost} />
                <Route path={`/create_community`} component={CreateCommunity} /> */}
                <Route
                  path={`/create_private_message`}
                  component={CreatePrivateMessage}
                />
                <Route
                  path={`/communities/page/:page`}
                  component={Communities}
                />
                <Route path={`/communities`} component={Communities} />
                <Route
                  path={`/post/:id/comment/:comment_id`}
                  component={Post}
                />
                <Route path={`/post/:id`} component={Post} />
                <Route
                  path={`/c/:name/data_type/:data_type/sort/:sort/page/:page`}
                  component={Community}
                />
                <Route path={`/community/:id`} component={Community} />
                <Route path={`/c/:name`} component={Community} />
                <Route
                  path={`/u/:username/view/:view/sort/:sort/page/:page`}
                  component={User}
                />
                <Route path={`/user/:id`} component={User} />
                <Route path={`/u/:username`} component={User} />
                <Route path={`/inbox`} component={Inbox} />
                <Route
                  path={`/modlog/community/:community_id`}
                  component={Modlog}
                />
                <Route path={`/modlog`} component={Modlog} />
                <Route path={`/setup`} component={Setup} />
                <Route path={`/admin`} component={AdminSettings} />
                <Route
                  path={`/search/q/:q/type/:type/sort/:sort/page/:page`}
                  component={Search}
                />
                <Route path={`/search`} component={Search} />
                <Route path={`/sponsors`} component={Sponsors} />
                <Route
                  path={`/password_change/:token`}
                  component={PasswordChange}
                />
              </Switch>
              <Symbols />
            </div>
            <Footer />
          </div>
        </BrowserRouter>
        </ApolloProvider>
      </Trans>
    );
  }
}

ReactDOM.render(
  <Index />,
  document.getElementById('root')
);
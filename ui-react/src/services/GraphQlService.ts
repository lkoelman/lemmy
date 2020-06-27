import { gqlUri } from '../env';
import { ApolloClient, InMemoryCache } from 'apollo-boost';
import { createHttpLink } from "apollo-link-http";

export const link = createHttpLink({
    uri: gqlUri
});

/**
 * Handle GraphQl requests through using Apollo
 */
export class GraphQlService {

  private static _instance: GraphQlService
  public client: ApolloClient<any>;

  private constructor() {
    this.client = new ApolloClient({
        cache: new InMemoryCache(),
        link,
    });
  }

  public static get Instance() {
    return this._instance || (this._instance = new this());
  }
}
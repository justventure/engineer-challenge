import { GraphQLClient } from 'graphql-request'

let clientInstance: GraphQLClient | null = null

export const getClient = (): GraphQLClient => {
  if (!clientInstance) {
    const url = typeof window === 'undefined'
      ? process.env.INTERNAL_GRAPHQL_URL!
      : process.env.NEXT_PUBLIC_GRAPHQL_URL!

    clientInstance = new GraphQLClient(url, {
      credentials: 'include',
    })
  }
  return clientInstance
}

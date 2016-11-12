use api::raw::{ResourceResponse, CollectionResponse, JobResponse, RelResponse, RawFetch};
use api::{AsyncAction, Error};
use router::{self, Response, Linker};

mod jsonapi;

pub use self::jsonapi::JsonApi;

pub trait Presenter<T: RawFetch>: Sized {
    type Response: Response;
    type Linker: Linker;
    type Include;
    fn prepare(field_set: Option<Vec<String>>, linker: Self::Linker) -> Self;
    fn present_resource(self, response: ResourceResponse<Self::Include, T>) -> Self::Response;
    fn present_collection(self, response: CollectionResponse<Self::Include, T>) -> Self::Response;
    fn present_rel(self, rel: RelResponse<Self::Include>) -> Self::Response;
    fn present_err(self, error: Error) -> Self::Response;

    fn try_present<R: Presentable<Self, T>>(self, result: Result<R, Error>) -> Self::Response {
        match result {
            Ok(response)    => response.present(self),
            Err(error)      => self.present_err(error),
        }
    }
}

pub trait ConvertInclude<T> {
    fn convert(attributes: T) -> Self;
}

pub trait Presentable<P: Presenter<T>, T: RawFetch> {
    fn present(self, presenter: P) -> P::Response;
}

impl<P, T> Presentable<P, T> for ResourceResponse<P::Include, T>
where
    P: Presenter<T>,
    T: RawFetch,
{
    fn present(self, presenter: P) -> P::Response {
        presenter.present_resource(self)
    }
}

impl<P, T> Presentable<P, T> for CollectionResponse<P::Include, T>
where
    P: Presenter<T>,
    T: RawFetch,
{
    fn present(self, presenter: P) -> P::Response {
        presenter.present_collection(self)
    }
} 

impl<P, T> Presentable<P, T::Job> for JobResponse<T>
where
    P: Presenter<T::Job>,
    T: AsyncAction,
{
    fn present(self, presenter: P) -> P::Response {
        let mut response = presenter.present_resource(ResourceResponse {
            resource: self.resource,
            includes: vec![],
        });
        response.set_status(router::Status::Accepted);
        //TODO set location header
        response
    }
}

impl<P, T> Presentable<P, T> for RelResponse<P::Include>
where
    P: Presenter<T>,
    T: RawFetch,
{
    fn present(self, presenter: P) -> P::Response {
        presenter.present_rel(self)
    }
}

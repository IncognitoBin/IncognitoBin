import BigLogo from "../assets/bigLogo.webp";

const NotFoundPage = ()=> {
    return(
        <div className="flex flex-col items-center justify-center min-h-[90dvh] bg-gray-950 dark:bg-gray-950 text-gray-50 p-4">
          <div className="max-w-xl w-full space-y-6 text-center">
            <img
              src={BigLogo}
              width={320}
              height={240}
              alt="404"
              className="mx-auto aspect-video rounded-lg object-cover "
            />
            <div className="space-y-2">
              <h1 className="text-4xl font-bold">
                Ehh? IDK But This Paste Doesn't Exist.
              </h1>
              <p className="text-gray-400">
                This paste may have been deleted or hasn't been created yet.
              </p>
            </div>
          </div>
        </div>
    )
}
export default NotFoundPage
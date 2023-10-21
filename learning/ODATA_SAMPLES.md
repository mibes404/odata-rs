# OData Tutorial

## 01. Read the service root
All REST APIs should have a single entry point from which a generic hypermedia client can navigate to the resources in the service. In the response we see links to three things: 1. The $metadata document that describes the schema of ther service 2. Links to various collections of objects like People and Airports 3. Links to other top-level items like Me (a singleton) and operations.

```http
GET https://services.odata.org/V4/TripPinService/ HTTP/1.1
```

## 02. Read the service metadata

$metadata is an endpoint in OData services that contains a machine-readable description of the service model including type schemas, available operations, etc.

```http
GET https://services.odata.org/V4/TripPinService/$metadata HTTP/1.1
```

## 03. Read an entity set

One of the most common responses from a REST API is a collection of resources. In this case we asked for the People collection. For each response, the OData service writes a self-described response (another REST principle) by annotating the response with a context URL. This context URL tells the service that the contents of the response are a collection of things in the People entity set. The @odata.nextLink annotation is present because the server opted to split the result set across multiple pages. The client can also drive paging using $top and $skip, but server-side paging is a mitigation against DoS attacks. The value property contains the bulk of the response. Note that @odata.id and @odata.editLink should generally not be present in the payload unless they deviate from convention. In this case it appears that there is a bug in our sample service. Pretend those properties aren't there.

```http
GET https://services.odata.org/V4/TripPinService/People HTTP/1.1
```

## 04. Get a single entity from an entity set

To get a particular entity from a collection, append a key segment. By default, key segments in OData services are bounded by parens because they may be composite keys, e.g., OrderLine(OrderId=1,LineNumber=1) or alternate keys, e.g., Person(SSN='000-00-0000') and Person(2115) both address the same resource. Some OData services use normal URL segments for key segments, e.g., Orders/1. This is not recommended because of the scenarios mentioned above.

```http
GET https://services.odata.org/V4/TripPinService/People('russellwhyte') HTTP/1.1
```

## 05. Get a primitive property

Even when fetching a primitive property, an object wrapper is returned rather than returning the raw primitive. This is to protect against a JSON vulnerability.

```http
GET https://services.odata.org/V4/TripPinService/People('russellwhyte')/FirstName HTTP/1.1
```

## 06. Get the raw value of a primitive property

If you really want the raw value of a primitive property, you can get it by appending $value to the URL. Note that the Content-Type header indicates that the content is text/plain.

```http
GET https://services.odata.org/V4/TripPinService/People('russellwhyte')/FirstName/$value HTTP/1.1
```


## 07. Navigate to related entities

To navigate the resource graph, keep appending segments representing valid property names as defined in $metadata or in a full metadata response (see query x). In this case we have started from the service root, navigated to the entity set People, navigated to the resource keyed 'russellwhyte', navigated to the Friends property, navigated to the resource keyed 'scottketchum', and finally navigated to the AddressInfo property. Note that the @odata.context URL self-describes the payload.

```http
GET https://services.odata.org/V4/TripPinService/People('russellwhyte')/Friends('scottketchum')/AddressInfo HTTP/1.1
```

## 08. Filter a collection

The $filter system query option can be used to filter any collection of resources. Note that the response to a filtered collection is a collection of the same type, regardless of the number of matched resources.

```http
GET https://services.odata.org/V4/TripPinService/People?$filter=FirstName eq 'Vincent' HTTP/1.1
```

## 09. Filter on enumeration properties

You can filter any type of collection in OData services. When referring to a member of enum properties, please don't ignore the namespace for the enum property.

```http
GET https://services.odata.org/V4/TripPinService/People?$filter=Gender eq Microsoft.OData.SampleService.Models.TripPin.PersonGender'Female' HTTP/1.1
```

## 10. Filter on nested structures

You can use any related properties in a filter clause by using the same segments used in the path to traverse properties.

```http
GET https://services.odata.org/V4/TripPinService/Airports?$filter=Location/City/Region eq 'California' HTTP/1.1
```

## 11. Filter using logic operators

You can use and, or and not to create more complex filter clauses.

```http
GET https://services.odata.org/V4/TripPinService/People?$filter=not(contains(FirstName,'Q')) and (AddressInfo/any(ai:ai/City/Region eq 'WA') or AddressInfo/any(ai:ai/City/Region eq 'ID')) HTTP/1.1
```

## 12. Filter using any/all operators

You can use any/all lambda-style filters for collection properties.

```http
GET https://services.odata.org/V4/TripPinService/People?$filter=Emails/any(e: endswith(e, 'contoso.com')) HTTP/1.1
```

## 13. Filter using built-in functions

Built-in functions may be leveraged to filter the collection.

```http
GET https://services.odata.org/V4/TripPinService/People('russellwhyte')/Trips(0)/PlanItems/Microsoft.OData.SampleService.Models.TripPin.Flight HTTP/1.1
```

## 14. Sort a collection

You can use the $orderby system query option to specify ordering criteria. You can use many of the functions usable in $filter in $orderby as well.

```http
GET https://services.odata.org/V4/TripPinService/People?$orderby=length(FirstName) desc,UserName HTTP/1.1
```

## 15. Client-side paging

There are two types of paging in OData services. Servers can enable server-side paging, returning nextLinks that clients can follow to subsequent pages. Clients can drive paging using $top and $skip.

```http
GET https://services.odata.org/V4/TripPinService/People?$top=2&$skip=2 HTTP/1.1
```

## 16. Counting the elements in a collection

If you want to know how many items meet a condition, you can use the $count path segment. Note that the Content-Type header indicates that the content is text/plain. Although it doesn't work with system query options in the reference service, $count can typically be combined with $filter.

```http
GET https://services.odata.org/V4/TripPinService/People/$count HTTP/1.1
```

## 17. Expand related entities

You can use the $expand system query option to include related resources. The expand clause can include many of the other system query options, enabling left-join type semantics. Also, you can expand <property>/$ref in order to get just the links to the related resources.

```http
GET https://services.odata.org/V4/TripPinService/People?$expand=Friends,Trips HTTP/1.1
```

## 18. Select the properties

You can use the $select system query option to only return specified properties.

```http
GET https://services.odata.org/V4/TripPinService/People?$select=FirstName,LastName HTTP/1.1
```

## 19. Request full metadata

By default OData services return an extremely compact JSON format. This happens by stripping out all of the metadata that should be calculable by "smart" OData clients. For generic hypermedia clients, you can request additional metadata by using the Accept header or $format system query option to request application/json;odata.metadata=full. In this case, we get a bunch of additional annotations in the payload indicating type information and relationships to related resources.

```http
GET https://services.odata.org/V4/TripPinService/People?$format=application/json;odata.metadata=full HTTP/1.1
```

## 20. Casting types

Another way of filtering items is to use a type cast segment. In this case we are looking for all of the flights that are part of Russell's trip. A type cast also allows us to append additional path segments that are properties of the subtype.

```http
GET https://services.odata.org/V4/TripPinService/People('russellwhyte')/Trips(0)/PlanItems/Microsoft.OData.SampleService.Models.TripPin.Flight HTTP/1.1
```

## 21. Create an entity

To create a resource, send a POST to a collection.

```http
POST https://services.odata.org/V4/TripPinService/People HTTP/1.1
content-type: application/json

{
    "UserName": "miathompson",
    "FirstName": "Mia",
    "LastName": "Thompson",
    "Gender": "Female"
}
```

## 22. Delete an entity

To remove a resource, send an HTTP DELETE to the resource URL. Note: since the People entity set has concurrency enabled, you will need to request the resource and set the If-Match header to the appropriate value to run this request.

```http
DELETE https://services.odata.org/V4/TripPinService/People('miathompson') HTTP/1.1
If-Match: W/"08DBCA2FB039A0A1"
```

## 23. Update an entity

To update a resource, send a PATCH request with the properties you wish to modify. You can also use PUT, but the semantics for put require all properties to be either sent on the wire or reverted to their default values. Note: since the People entity set has concurrency enabled, you will need to request the resource and set the If-Match header to the appropriate value to run this request.

```http
PATCH https://services.odata.org/V4/TripPinService/People('miathompson') HTTP/1.1
If-Match: W/"08DBCA2FB039A0A1"
content-type: application/json

{
    "Emails": ["miathompson@contoso.com", "miathompson@example.com"]
}
```

## 24. Invoke an unbound function

OData support unbound custom functions. The unbound functions can be invoked either with parameters or not and unbound functions can be viewed in the $metadata. Note: OData functions CANNOT have side effect, so only GET verb is allowed.

```http
GET https://services.odata.org/V4/TripPinService/GetNearestAirport(lat = 33, lon = -118) HTTP/1.1
```

## 25. Invoke a bound function

OData support bound custom functions. The bound functions are bounded to a resource. Note: OData functions CANNOT have side effect, so only GET verb is allowed.

```http
GET http://services.odata.org/V4/TripPinService/People('russellwhyte')/Microsoft.OData.SampleService.Models.TripPin.GetFavoriteAirline() HTTP/1.1
```

## 26. Invoke a bound action

OData support bound custom actions. Actions can have side effects. So the HTTP verb for an OData action can be GET,POST,PUT,DELETE according to the behavior of the action. In the example below, the action implies a POST.

```http
POST https://services.odata.org/V4/TripPinService/People('russellwhyte')/Microsoft.OData.SampleService.Models.TripPin.ShareTrip HTTP/1.1
Content-Type: application/json

{
    "userName": "scottketchum",
    "tripId": 0
}
```